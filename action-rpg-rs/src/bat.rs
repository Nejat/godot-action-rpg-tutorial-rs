use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::{assume_safe, call, child_node, get_parameter, load_resource, set_parameter};
use crate::has_effect::HasEffect;
use crate::hurt_box::METHOD_PLAY_HIT_EFFECT;
use crate::player_detection::{METHOD_CAN_SEE_PLAYER, METHOD_GET_PLAYER};
use crate::stats::PROPERTY_HEALTH;
use crate::sword::{PROPERTY_DAMAGE, PROPERTY_KNOCK_BACK_VECTOR};
use crate::soft_collision::{METHOD_IS_COLLIDING, METHOD_GET_PUSH_VECTOR};

pub(crate) const PROPERTY_ACCELERATION: &str = "acceleration";
pub(crate) const PROPERTY_FRICTION: &str = "friction";
pub(crate) const PROPERTY_KNOCK_BACK_FORCE: &str = "knock_back_force";
pub(crate) const PROPERTY_MAX_SPEED: &str = "max_speed";
pub(crate) const PROPERTY_PUSH_VECTOR_FORCE: &str = "push_vector_force";

const DEFAULT_ACCELERATION: f32 = 300.0;
const DEFAULT_FRICTION: f32 = 200.0;
const DEFAULT_KNOCK_BACK_FORCE: f32 = 120.0;
const DEFAULT_MAX_SPEED: f32 = 50.0;
const DEFAULT_PUSH_VECTOR_FORCE: f32 = 400.0;

#[allow(dead_code)]
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
    effect: Option<Ref<PackedScene>>,
    #[property]
    friction: f32,
    hurtbox: Option<Ref<Node2D>>,
    knock_back: Vector2,
    #[property]
    knock_back_force: f32,
    #[property]
    max_speed: f32,
    player_detection: Option<Ref<Area2D>>,
    push_vector_force: f32,
    soft_collision: Option<Ref<Area2D>>,
    sprite: Option<Ref<AnimatedSprite>>,
    state: BatState,
    stats: Option<Ref<Node>>,
    velocity: Vector2,
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
            effect: None,
            friction: DEFAULT_FRICTION,
            hurtbox: None,
            knock_back: Vector2::zero(),
            knock_back_force: DEFAULT_KNOCK_BACK_FORCE,
            max_speed: DEFAULT_MAX_SPEED,
            player_detection: None,
            push_vector_force: DEFAULT_PUSH_VECTOR_FORCE,
            soft_collision: None,
            sprite: None,
            state: BatState::IDLE,
            stats: None,
            velocity: Vector2::zero(),
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
            .with_setter(|s: &mut Self, _, value: f32| s.max_speed = value)
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

        self.hurtbox = Some(child_node!(claim owner["HurtBox"]: Node2D));
        self.player_detection = Some(child_node!(claim owner["PlayerDetectionZone"]: Area2D));
        self.soft_collision = Some(child_node!(claim owner["SoftCollision"]: Area2D));
        self.sprite = Some(child_node!(claim owner["AnimatedSprite"]: AnimatedSprite));
        self.stats = Some(child_node!(owner["Stats"]));
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
                    let mut direction = unsafe { player.assume_safe().global_position() } - owner.global_position();
                    if direction != Vector2::zero() {
                        direction = direction.normalize();
                    }
                    self.velocity = self.velocity.move_towards(direction * self.max_speed, self.acceleration * delta);
                } else {
                    self.state = BatState::IDLE
                }

                assume_safe!(self.sprite).set_flip_h(self.velocity.lower_than(Vector2::zero()).x);
            }
            BatState::IDLE => {
                self.velocity = self.velocity.move_towards(Vector2::zero(), self.friction * delta);
                self.seek_player(owner);
            }
            BatState::WANDER => {}
        }

        if call!(self.soft_collision; METHOD_IS_COLLIDING).to_bool() {
            self.velocity += call!(self.soft_collision; METHOD_GET_PUSH_VECTOR).to_vector2() * delta * self.push_vector_force;
        }

        owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }

    fn seek_player(&mut self, _owner: &KinematicBody2D) {
        let can_see_player = call!(self.player_detection; METHOD_CAN_SEE_PLAYER).to_bool();
        if can_see_player {
            self.state = BatState::CHASE
        }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        let damage = get_parameter!(area[PROPERTY_DAMAGE]).to_i64();
        let health = get_parameter!(self.stats.unwrap(); PROPERTY_HEALTH).to_i64();

        set_parameter!(self.stats.unwrap(); PROPERTY_HEALTH = health - damage);

        self.knock_back = get_parameter!(area[PROPERTY_KNOCK_BACK_VECTOR]).to_vector2() * self.knock_back_force;

        call!(self.hurtbox; METHOD_PLAY_HIT_EFFECT);
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
