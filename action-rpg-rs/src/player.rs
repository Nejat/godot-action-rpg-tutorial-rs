use godot::classes::{
    AnimationNodeStateMachinePlayback, AnimationPlayer, AnimationTree, Area2D, CharacterBody2D,
    ICharacterBody2D, Input, PackedScene,
};
use godot::prelude::*;

use crate::has_effect::HasEffect;
use crate::hurt_box::HurtBox;
use crate::stats::PROPERTY_HEALTH;
use crate::sword::PROPERTY_KNOCK_BACK_VECTOR;

const DEFAULT_ACCELERATION: f32 = 500.0;
const DEFAULT_FRICTION: f32 = 500.0;
const DEFAULT_MAX_SPEED: f32 = 80.0;
const DEFAULT_ROLL_SPEED: f32 = 120.0;

const INPUT_ATTACK: &str = "ui_attack";
const INPUT_DOWN: &str = "ui_down";
const INPUT_LEFT: &str = "ui_left";
const INPUT_RIGHT: &str = "ui_right";
const INPUT_ROLL: &str = "ui_roll";
const INPUT_UP: &str = "ui_up";

const TRAVEL_ATTACK: &str = "Attack";
const TRAVEL_IDLE: &str = "Idle";
const TRAVEL_ROLL: &str = "Roll";
const TRAVEL_RUN: &str = "Run";

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum PlayerState {
    Attack,
    Move,
    Roll,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    #[var]
    pub acceleration: f32,
    animation_state: Option<Gd<AnimationNodeStateMachinePlayback>>,
    animation_tree: Option<Gd<AnimationTree>>,
    blink_animation: Option<Gd<AnimationPlayer>>,
    #[var]
    pub friction: f32,
    hurt_box: Option<Gd<Area2D>>,
    hurt_sound: Option<Gd<PackedScene>>,
    effect: Option<Gd<PackedScene>>,
    #[var]
    pub max_speed: f32,
    player_stats: Option<Gd<Node>>,
    #[var]
    pub roll_speed: f32,
    roll_vector: Vector2,
    state: PlayerState,
    sword: Option<Gd<Area2D>>,
    velocity: Vector2,
}

impl HasEffect for Player {
    fn effect_scene(&self) -> &Option<Gd<PackedScene>> {
        &self.effect
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Player {
            base,
            acceleration: DEFAULT_ACCELERATION,
            animation_state: None,
            animation_tree: None,
            blink_animation: None,
            friction: DEFAULT_FRICTION,
            hurt_box: None,
            hurt_sound: None,
            effect: None,
            max_speed: DEFAULT_MAX_SPEED,
            roll_speed: DEFAULT_ROLL_SPEED,
            roll_vector: Vector2::new(0.0, 1.0), // DOWN
            player_stats: None,
            state: PlayerState::Move,
            sword: None,
            velocity: Vector2::ZERO,
        }
    }

    fn ready(&mut self) {
        let mut animation_tree = self.base().get_node_as::<AnimationTree>("AnimationTree");

        let animation_state: Gd<AnimationNodeStateMachinePlayback> = animation_tree
            .get("parameters/playback")
            .try_to()
            .expect("AnimationNodeStateMachinePlayback from parameters/playback");

        animation_tree.set_active(true);

        self.animation_tree = Some(animation_tree);
        self.animation_state = Some(animation_state);

        self.blink_animation = Some(
            self.base()
                .get_node_as::<AnimationPlayer>("BlinkAnimationPlayer"),
        );

        self.hurt_box = Some(self.base().get_node_as::<Area2D>("HurtBox"));
        self.sword = Some(self.base().get_node_as::<Area2D>("HitboxPivot/SwordHitbox"));
        self.hurt_sound = Some(load::<PackedScene>("res://Player/PlayerHurtSound.tscn"));
        self.effect = Some(load::<PackedScene>("res://Effects/EnemyDeathEffect.tscn"));

        let player_stats = self.base().get_node_as::<Node>("/root/PlayerStats");

        self.player_stats = Some(player_stats);
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;

        match self.state {
            PlayerState::Move => self.move_state(delta),
            PlayerState::Attack => self.attack_state(),
            PlayerState::Roll => self.roll_state(),
        }
    }
}

#[godot_api]
impl Player {
    #[func]
    fn attack_animation_finished(&mut self) {
        self.state = PlayerState::Move;
    }

    #[func]
    fn roll_animation_finished(&mut self) {
        self.velocity *= 0.8; // ease sliding past roll animation
        self.state = PlayerState::Move;
    }

    #[inline]
    fn attack_state(&mut self) {
        self.velocity = Vector2::ZERO;

        if let Some(anim_state) = self.animation_state.as_mut() {
            anim_state.travel(TRAVEL_ATTACK);
        }
    }

    #[inline]
    fn move_state(&mut self, delta: f32) {
        let input = Input::singleton();
        let mut input_vector = Vector2::ZERO;

        input_vector.x =
            input.get_action_strength(INPUT_RIGHT) - input.get_action_strength(INPUT_LEFT);

        input_vector.y =
            input.get_action_strength(INPUT_DOWN) - input.get_action_strength(INPUT_UP);

        if input_vector != Vector2::ZERO {
            input_vector = input_vector.normalized();

            self.roll_vector = input_vector;

            if let Some(sword) = self.sword.as_mut() {
                sword.set(PROPERTY_KNOCK_BACK_VECTOR, &input_vector.to_variant());
            }

            if let Some(anim_tree) = self.animation_tree.as_mut() {
                anim_tree.set("parameters/Idle/blend_position", &input_vector.to_variant());
                anim_tree.set("parameters/Run/blend_position", &input_vector.to_variant());

                anim_tree.set(
                    "parameters/Attack/blend_position",
                    &input_vector.to_variant(),
                );

                anim_tree.set("parameters/Roll/blend_position", &input_vector.to_variant());
            }

            if let Some(anim_state) = self.animation_state.as_mut() {
                anim_state.travel(TRAVEL_RUN);
            }

            self.velocity = self
                .velocity
                .move_toward(input_vector * self.max_speed, self.acceleration * delta);
        } else {
            if let Some(anim_state) = self.animation_state.as_mut() {
                anim_state.travel(TRAVEL_IDLE);
            }

            self.velocity = self
                .velocity
                .move_toward(Vector2::ZERO, self.friction * delta);
        }

        self.move_player();

        if input.is_action_just_pressed(INPUT_ROLL) {
            self.state = PlayerState::Roll;
        }

        if input.is_action_just_pressed(INPUT_ATTACK) {
            self.state = PlayerState::Attack;
        }
    }

    #[inline]
    fn roll_state(&mut self) {
        self.velocity = self.roll_vector * self.roll_speed;

        if let Some(anim_state) = self.animation_state.as_mut() {
            anim_state.travel(TRAVEL_ROLL);
        }

        self.move_player();
    }

    #[inline]
    fn move_player(&mut self) {
        let vel = self.velocity;

        self.base_mut().set_velocity(vel);
        self.base_mut().move_and_slide();

        self.velocity = self.base().get_velocity();
    }

    #[func]
    fn _on_hurt_box_area_entered(&mut self, _area: Gd<Area2D>) {
        // enemy hit box does not have damage, the video "fix" causes a bug
        if let Some(player_stats) = self.player_stats.as_mut() {
            let current_health = player_stats
                .get(PROPERTY_HEALTH)
                .try_to::<i64>()
                .unwrap_or(0);

            let new_health = current_health - 1;

            player_stats.set(PROPERTY_HEALTH, &new_health.to_variant());

            let verify_health = player_stats
                .get(PROPERTY_HEALTH)
                .try_to::<i64>()
                .unwrap_or(0);

            // Manually update the health UI since signals may not be working
            {
                let tree = self.base().get_tree();

                if let Some(current_scene) = tree.get_current_scene()
                    && let Some(mut health_ui) =
                        current_scene.get_node_or_null("CanvasLayer/HealthUI")
                {
                    health_ui.call("set_hearts", &[verify_health.to_variant()]);
                }
            }

            // If health is 0 or below, player death
            if verify_health <= 0 {
                let owner: Gd<Node2D> = self.base().clone().upcast();

                self.play_effect_parent(&owner);
                self.base_mut().queue_free();

                return;
            }
        }

        if let Some(hurt_box) = self.hurt_box.as_ref() {
            let mut hb = hurt_box.clone().cast::<HurtBox>();

            hb.bind_mut().start_invincibility(0.5);
            hb.bind_mut().play_hit_effect();
        }

        // Spawn hurt sound
        if let Some(hurt_sound_scene) = self.hurt_sound.as_ref()
            && let Some(instance) = hurt_sound_scene.instantiate()
        {
            let tree = self.base().get_tree();

            if let Some(mut current_scene) = tree.get_current_scene() {
                current_scene.add_child(&instance);
            }
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
        self.base_mut().queue_free();
    }
}
