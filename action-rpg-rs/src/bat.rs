use godot::classes::{
    AnimatedSprite2D, AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D, PackedScene,
    RandomNumberGenerator,
};
use godot::prelude::*;

use crate::has_effect::HasEffect;
use crate::hurt_box::HurtBox;
use crate::player_detection::PlayerDetectionZone;
use crate::soft_collision::SoftCollision;
use crate::stats::PROPERTY_HEALTH;
use crate::sword::{PROPERTY_DAMAGE, PROPERTY_KNOCK_BACK_VECTOR};
use crate::wander::WanderController;

// ratio of max speed to buffer the bat's approach to its target
const WANDER_BUFFER_RATIO: f32 = 0.08;

const DEFAULT_ACCELERATION: f32 = 300.0;
const DEFAULT_FRICTION: f32 = 200.0;
const DEFAULT_KNOCK_BACK_FORCE: f32 = 120.0;
const DEFAULT_MAX_SPEED: f32 = 50.0;
const DEFAULT_PUSH_VECTOR_FORCE: f32 = 400.0;
const DEFAULT_WANDER_BUFFER_ZONE: f32 = DEFAULT_MAX_SPEED * WANDER_BUFFER_RATIO;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum BatState {
    Chase,
    Idle,
    Wander,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Bat {
    base: Base<CharacterBody2D>,
    #[var]
    pub acceleration: f32,
    blink_animation: Option<Gd<AnimationPlayer>>,
    effect: Option<Gd<PackedScene>>,
    #[var]
    pub friction: f32,
    hurt_box: Option<Gd<Area2D>>,
    knock_back: Vector2,
    #[var]
    pub knock_back_force: f32,
    #[var]
    pub max_speed: f32,
    player_detection: Option<Gd<Area2D>>,
    #[var]
    pub push_vector_force: f32,
    rand: Gd<RandomNumberGenerator>,
    soft_collision: Option<Gd<Area2D>>,
    sprite: Option<Gd<AnimatedSprite2D>>,
    state: BatState,
    stats: Option<Gd<Node>>,
    velocity: Vector2,
    wander_buffer_zone: f32,
    wander_controller: Option<Gd<Node2D>>,
}

impl HasEffect for Bat {
    fn effect_scene(&self) -> &Option<Gd<PackedScene>> {
        &self.effect
    }
}

#[godot_api]
impl ICharacterBody2D for Bat {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Bat {
            base,
            acceleration: DEFAULT_ACCELERATION,
            blink_animation: None,
            effect: None,
            friction: DEFAULT_FRICTION,
            hurt_box: None,
            knock_back: Vector2::ZERO,
            knock_back_force: DEFAULT_KNOCK_BACK_FORCE,
            max_speed: DEFAULT_MAX_SPEED,
            player_detection: None,
            push_vector_force: DEFAULT_PUSH_VECTOR_FORCE,
            rand: RandomNumberGenerator::new_gd(),
            soft_collision: None,
            sprite: None,
            state: BatState::Idle,
            stats: None,
            velocity: Vector2::ZERO,
            wander_buffer_zone: DEFAULT_WANDER_BUFFER_ZONE,
            wander_controller: None,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;

        // knock back
        self.knock_back = self
            .knock_back
            .move_toward(Vector2::ZERO, self.friction * delta);

        let kb = self.knock_back;

        self.base_mut().set_velocity(kb);
        self.base_mut().move_and_slide();

        self.knock_back = self.base().get_velocity();

        match self.state {
            BatState::Chase => {
                let can_see = self
                    .player_detection
                    .as_ref()
                    .map(|pd| {
                        pd.clone()
                            .cast::<PlayerDetectionZone>()
                            .bind()
                            .can_see_player()
                    })
                    .unwrap_or(false);

                if can_see {
                    let player_variant = self
                        .player_detection
                        .as_ref()
                        .map(|pd| pd.clone().cast::<PlayerDetectionZone>().bind().get_player())
                        .unwrap_or(Variant::nil());

                    if let Ok(player) = player_variant.try_to::<Gd<Node2D>>() {
                        let my_pos = self.base().get_global_position();
                        let player_pos = player.get_global_position();
                        let direction = my_pos.direction_to(player_pos);
                        self.accelerate_towards(direction, delta);
                    } else {
                        self.state = BatState::Idle;
                    }
                } else {
                    self.state = BatState::Idle;
                }
            }
            BatState::Idle => {
                self.seek_player();
                self.next_state_on_finish(3.0);

                self.velocity = self
                    .velocity
                    .move_toward(Vector2::ZERO, self.friction * delta);
            }
            BatState::Wander => {
                self.seek_player();
                self.next_state_on_finish(3.0);

                let target_position = self
                    .wander_controller
                    .as_ref()
                    .map(|wc| wc.clone().cast::<WanderController>().bind().target_position)
                    .unwrap_or(Vector2::ZERO);

                let my_pos = self.base().get_global_position();
                let direction = my_pos.direction_to(target_position);

                self.accelerate_towards(direction, delta);

                if my_pos.distance_to(target_position) <= self.wander_buffer_zone {
                    self.next_state(3.0);
                }
            }
        }

        // soft collision push
        let is_colliding = self
            .soft_collision
            .as_ref()
            .map(|sc| sc.clone().cast::<SoftCollision>().bind().is_colliding())
            .unwrap_or(false);

        if is_colliding {
            let push_vector = self
                .soft_collision
                .as_ref()
                .map(|sc| sc.clone().cast::<SoftCollision>().bind().get_push_vector())
                .unwrap_or(Vector2::ZERO);

            self.velocity += push_vector * delta * self.push_vector_force;
        }

        // flip sprite based on the movement direction
        if self.velocity != Vector2::ZERO
            && let Some(sprite) = self.sprite.as_mut()
        {
            sprite.set_flip_h(self.velocity.x < 0.0);
        }

        let vel = self.velocity;

        self.base_mut().set_velocity(vel);
        self.base_mut().move_and_slide();
    }

    fn ready(&mut self) {
        self.effect = Some(load::<PackedScene>("res://Effects/EnemyDeathEffect.tscn"));

        self.blink_animation = Some(
            self.base()
                .get_node_as::<AnimationPlayer>("BlinkAnimationPlayer"),
        );

        self.hurt_box = Some(self.base().get_node_as::<Area2D>("HurtBox"));
        self.player_detection = Some(self.base().get_node_as::<Area2D>("PlayerDetectionZone"));
        self.soft_collision = Some(self.base().get_node_as::<Area2D>("SoftCollision"));

        self.sprite = Some(
            self.base()
                .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D"),
        );

        self.stats = Some(self.base().get_node_as::<Node>("Stats"));
        self.wander_controller = Some(self.base().get_node_as::<Node2D>("WanderController"));
        self.state = self.pick_random_state();
    }
}

#[godot_api]
impl Bat {
    #[inline]
    fn accelerate_towards(&mut self, direction: Vector2, delta: f32) {
        self.velocity = self
            .velocity
            .move_toward(direction * self.max_speed, self.acceleration * delta);
    }

    #[inline]
    fn next_state(&mut self, max_secs: f64) {
        self.state = self.pick_random_state();

        if let Some(wc) = self.wander_controller.as_ref() {
            let duration = self.rand.randf_range(1.0, max_secs as f32);

            wc.clone()
                .cast::<WanderController>()
                .bind_mut()
                .start_timer(duration as f64);
        }
    }

    #[inline]
    fn next_state_on_finish(&mut self, max_secs: f64) {
        let timer_complete = self
            .wander_controller
            .as_ref()
            .map(|wc| {
                wc.clone()
                    .cast::<WanderController>()
                    .bind()
                    .is_timer_complete()
            })
            .unwrap_or(true);

        if timer_complete {
            self.next_state(max_secs);
        }
    }

    #[inline]
    fn seek_player(&mut self) {
        let can_see = self
            .player_detection
            .as_ref()
            .map(|pd| {
                pd.clone()
                    .cast::<PlayerDetectionZone>()
                    .bind()
                    .can_see_player()
            })
            .unwrap_or(false);

        if can_see {
            self.state = BatState::Chase;
        }
    }

    #[inline]
    fn pick_random_state(&mut self) -> BatState {
        if self.rand.randi_range(1, 2) == 1 {
            BatState::Idle
        } else {
            BatState::Wander
        }
    }

    #[func]
    fn _on_hurt_box_area_entered(&mut self, area: Gd<Area2D>) {
        let damage = area.get(PROPERTY_DAMAGE).try_to::<i64>().unwrap_or(0);

        if let Some(stats) = self.stats.as_mut() {
            let current_health = stats.get(PROPERTY_HEALTH).try_to::<i64>().unwrap_or(0);
            let new_health = current_health - damage;

            stats.set(PROPERTY_HEALTH, &new_health.to_variant());

            let verify_health = stats.get(PROPERTY_HEALTH).try_to::<i64>().unwrap_or(0);

            if verify_health <= 0 {
                let owner: Gd<Node2D> = self.base().clone().upcast();

                self.play_effect_parent(&owner);
                self.base_mut().queue_free();

                return;
            }
        }

        self.knock_back = area
            .get(PROPERTY_KNOCK_BACK_VECTOR)
            .try_to::<Vector2>()
            .unwrap_or(Vector2::ZERO)
            * self.knock_back_force;

        if let Some(hurt_box) = self.hurt_box.as_ref() {
            let mut hb = hurt_box.clone().cast::<HurtBox>();

            hb.bind_mut().start_invincibility(0.4);
            hb.bind_mut().play_hit_effect();
        }
    }

    #[func]
    fn _on_hurt_box_invincibility_ended(&mut self) {
        if let Some(anim) = self.blink_animation.as_mut() {
            anim.play_ex().name("Stop").done();
        }
    }

    #[func]
    fn _on_hurt_box_invincibility_started(&mut self) {
        if let Some(anim) = self.blink_animation.as_mut() {
            anim.play_ex().name("Start").done();
        }
    }

    #[func]
    fn _on_stats_no_health(&mut self) {
        let owner: Gd<Node2D> = self.base().clone().upcast();

        self.play_effect_parent(&owner);
        self.base_mut().queue_free();
    }
}
