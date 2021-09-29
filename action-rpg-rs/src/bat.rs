use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::{assume_safe, call, child_node, get_parameter, load_resource, set_parameter};
use crate::has_effect::HasEffect;
use crate::hurt_box::{METHOD_PLAY_HIT_EFFECT, METHOD_START_INVINCIBILITY};
use crate::player_detection::{METHOD_CAN_SEE_PLAYER, METHOD_GET_PLAYER};
use crate::soft_collision::{METHOD_GET_PUSH_VECTOR, METHOD_IS_COLLIDING};
use crate::stats::PROPERTY_HEALTH;
use crate::sword::{PROPERTY_DAMAGE, PROPERTY_KNOCK_BACK_VECTOR};
use crate::wander::{METHOD_IS_TIMER_COMPLETE, METHOD_START_TIMER, PROPERTY_TARGET_POSITION};

pub(crate) const PROPERTY_ACCELERATION: &str = "acceleration";
pub(crate) const PROPERTY_FRICTION: &str = "friction";
pub(crate) const PROPERTY_KNOCK_BACK_FORCE: &str = "knock_back_force";
pub(crate) const PROPERTY_MAX_SPEED: &str = "max_speed";
pub(crate) const PROPERTY_PUSH_VECTOR_FORCE: &str = "push_vector_force";

// i choose this ratio of max speed to buffer the bat's approach to it's target
const WANDER_BUFFER_RATIO: f32 = 0.08; // this value might be frame rate dependent

const DEFAULT_ACCELERATION: f32 = 300.0;
const DEFAULT_FRICTION: f32 = 200.0;
const DEFAULT_KNOCK_BACK_FORCE: f32 = 120.0;
const DEFAULT_MAX_SPEED: f32 = 50.0;
const DEFAULT_PUSH_VECTOR_FORCE: f32 = 400.0;
const DEFAULT_WANDER_BUFFER_ZONE: f32 = DEFAULT_MAX_SPEED * WANDER_BUFFER_RATIO;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum BatState {
    CHASE,
    IDLE,
    WANDER,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register)]
pub struct Bat {
    #[property]
    acceleration: f32,
    blink_animation: Option<Ref<AnimationPlayer>>,
    effect: Option<Ref<PackedScene>>,
    #[property]
    friction: f32,
    hurt_box: Option<Ref<Node2D>>,
    knock_back: Vector2,
    #[property]
    knock_back_force: f32,
    #[property]
    max_speed: f32,
    player_detection: Option<Ref<Area2D>>,
    push_vector_force: f32,
    rand: Ref<RandomNumberGenerator, Unique>,
    soft_collision: Option<Ref<Area2D>>,
    sprite: Option<Ref<AnimatedSprite>>,
    state: BatState,
    stats: Option<Ref<Node>>,
    velocity: Vector2,
    wander_buffer_zone: f32,
    wander_controller: Option<Ref<Node2D>>,
}

impl HasEffect for Bat {
    fn effect_scene(&self) -> &Option<Ref<PackedScene>> {
        &self.effect
    }
}

impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            acceleration: DEFAULT_ACCELERATION,
            blink_animation: None,
            effect: None,
            friction: DEFAULT_FRICTION,
            hurt_box: None,
            knock_back: Vector2::zero(),
            knock_back_force: DEFAULT_KNOCK_BACK_FORCE,
            max_speed: DEFAULT_MAX_SPEED,
            player_detection: None,
            push_vector_force: DEFAULT_PUSH_VECTOR_FORCE,
            rand: RandomNumberGenerator::new(),
            soft_collision: None,
            sprite: None,
            state: BatState::IDLE,
            stats: None,
            velocity: Vector2::zero(),
            wander_buffer_zone: DEFAULT_WANDER_BUFFER_ZONE,
            wander_controller: None,
        }
    }

    //noinspection DuplicatedCode
    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<f32>(PROPERTY_ACCELERATION)
            .with_getter(|s: &Self, _| s.acceleration)
            .with_setter(|s: &mut Self, _, value: f32| s.acceleration = value)
            .with_default(DEFAULT_ACCELERATION)
            .done();

        builder
            .add_property::<f32>(PROPERTY_FRICTION)
            .with_getter(|s: &Self, _| s.friction)
            .with_setter(|s: &mut Self, _, value: f32| s.friction = value)
            .with_default(DEFAULT_FRICTION)
            .done();

        builder
            .add_property::<f32>(PROPERTY_KNOCK_BACK_FORCE)
            .with_getter(|s: &Self, _| s.knock_back_force)
            .with_setter(|s: &mut Self, _, value: f32| s.knock_back_force = value)
            .with_default(DEFAULT_KNOCK_BACK_FORCE)
            .done();

        builder
            .add_property::<f32>(PROPERTY_MAX_SPEED)
            .with_getter(|s: &Self, _| s.max_speed)
            .with_setter(|s: &mut Self, _, value: f32| {
                s.max_speed = value;
                s.wander_buffer_zone = s.max_speed * WANDER_BUFFER_RATIO;
            })
            .with_default(DEFAULT_MAX_SPEED)
            .done();

        builder
            .add_property::<f32>(PROPERTY_PUSH_VECTOR_FORCE)
            .with_getter(|s: &Self, _| s.push_vector_force)
            .with_setter(|s: &mut Self, _, value: f32| s.push_vector_force = value)
            .with_default(DEFAULT_PUSH_VECTOR_FORCE)
            .done();
    }
}

#[methods]
impl Bat {
    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        load_resource! { scene: PackedScene = "Effects/EnemyDeathEffect.tscn" {
            self.effect = Some(scene.claim())
        } }

        self.blink_animation = Some(child_node!(claim owner["BlinkAnimationPlayer"]: AnimationPlayer));
        self.hurt_box = Some(child_node!(claim owner["HurtBox"]: Node2D));
        self.player_detection = Some(child_node!(claim owner["PlayerDetectionZone"]: Area2D));
        self.soft_collision = Some(child_node!(claim owner["SoftCollision"]: Area2D));
        self.sprite = Some(child_node!(claim owner["AnimatedSprite"]: AnimatedSprite));
        self.stats = Some(child_node!(owner["Stats"]));
        self.wander_controller = Some(child_node!(claim owner["WanderController"]: Node2D));

        self.state = self.pick_random_state();
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        self.knock_back = owner.move_and_slide(
            self.knock_back.move_towards(Vector2::zero(), self.friction * delta),
            Vector2::zero(), false, 4, FRAC_PI_4, true,
        );

        match self.state {
            BatState::CHASE => {
                let player = call!(self.player_detection; METHOD_GET_PLAYER: KinematicBody2D);

                if let Some(player) = player {
                    let direction = owner.global_position()
                        .direction_to(unsafe { player.assume_safe() }.global_position());

                    self.accelerate_towards(direction, delta);
                } else {
                    self.state = BatState::IDLE
                }
            }
            BatState::IDLE => {
                self.seek_player(owner);
                self.next_state_on_finish(3.0);

                self.velocity = self.velocity.move_towards(Vector2::zero(), self.friction * delta);
            }
            BatState::WANDER => {
                self.seek_player(owner);
                self.next_state_on_finish(3.0);

                let target_position = get_parameter!(
                    self.wander_controller.unwrap(); PROPERTY_TARGET_POSITION
                ).to_vector2();

                let direction = owner.global_position().direction_to(target_position);

                self.accelerate_towards(direction, delta);

                if owner.global_position().distance_to(target_position) <= self.wander_buffer_zone {
                    self.next_state(3.0);
                }
            }
        }

        if call!(self.soft_collision; METHOD_IS_COLLIDING).to_bool() {
            self.velocity += call!(self.soft_collision; METHOD_GET_PUSH_VECTOR).to_vector2()
                * delta * self.push_vector_force;
        }

        // move flip logic here for all movement states
        // check for stopped bat to keep last direction
        if self.velocity != Vector2::zero() {
            assume_safe!(self.sprite).set_flip_h(self.velocity.x < 0.0);
        }

        owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }

    #[inline]
    fn accelerate_towards(&mut self, direction: Vector2, delta: f32) {
        self.velocity = self.velocity.move_towards(
            direction * self.max_speed,
            self.acceleration * delta,
        );
    }

    #[inline]
    fn next_state(&mut self, max_secs: f64) {
        self.state = self.pick_random_state();

        call!(
            self.wander_controller;
            METHOD_START_TIMER(self.rand.randf_range(1.0, max_secs).to_variant())
        );
    }

    #[inline]
    fn next_state_on_finish(&mut self, max_secs: f64) {
        let timer_complete = call!(self.wander_controller; METHOD_IS_TIMER_COMPLETE).to_bool();

        if timer_complete {
            self.next_state(max_secs);
        }
    }

    #[inline]
    fn seek_player(&mut self, _owner: &KinematicBody2D) {
        let can_see_player = call!(self.player_detection; METHOD_CAN_SEE_PLAYER).to_bool();
        if can_see_player {
            self.state = BatState::CHASE
        }
    }

    // this did not need the overhead of lists and list manipulation,
    // so this is my simplified solution
    #[inline]
    fn pick_random_state(&mut self) -> BatState {
        if self.rand.randi_range(1, 2) == 1 {
            BatState::IDLE
        } else {
            BatState::WANDER
        }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        let damage = get_parameter!(area[PROPERTY_DAMAGE]).to_i64();
        let stats = self.stats.unwrap();
        let health = get_parameter!(stats; PROPERTY_HEALTH).to_i64();

        set_parameter!(stats; PROPERTY_HEALTH = health - damage);

        self.knock_back = get_parameter!(area[PROPERTY_KNOCK_BACK_VECTOR]).to_vector2()
            * self.knock_back_force;

        call!(self.hurt_box; METHOD_START_INVINCIBILITY(0.4.to_variant()));
        call!(self.hurt_box; METHOD_PLAY_HIT_EFFECT);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_invincibility_ended(&self, _owner: &KinematicBody2D) {
        assume_safe!(self.blink_animation).play("Stop", -1.0, 1.0, false);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_invincibility_started(&self, _owner: &KinematicBody2D) {
        assume_safe!(self.blink_animation).play("Start", -1.0, 1.0, false);
    }

    // when connecting signal in the godot editor, click the "advanced" switch
    // and select the "deferred" option, otherwise an exception occurs
    // todo: figure out why this is necessary
    #[export]
    #[allow(non_snake_case)]
    fn _on_Stats_no_health(&self, owner: &KinematicBody2D) {
        self.play_effect_parent(owner);
        owner.queue_free();
    }
}
