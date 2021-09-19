use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;
use crate::get_parameter;

type AnimationPlayback = AnimationNodeStateMachinePlayback;

#[allow(dead_code)]
enum PlayerState {
    Attack,
    Move,
    Roll,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
    state: PlayerState,
}

const ACCELERATION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;
const FRICTION: f32 = 500.0;

impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
            state: PlayerState::Move,
        }
    }
}

// the additional values passed to godot functions, that are not mentioned in
// the video, are listed in the api documentation and are defaults in gdscript

#[methods]
impl Player {
    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        child_node! { animation_tree: AnimationTree = owner["AnimationTree"] }

        animation_tree.set_active(true);
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        match self.state {
            PlayerState::Move =>
                self.move_state(owner, delta),
            PlayerState::Attack =>
                self.attack_state(owner, delta),
            PlayerState::Roll => {}
        }
    }

    #[export]
    fn attack_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.state = PlayerState::Move
    }

    #[export]
    fn roll_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.state = PlayerState::Move
    }

    #[inline]
    fn attack_state(&mut self, owner: &KinematicBody2D, _delta: f32) {
        child_node! { animation_tree: AnimationTree = owner["AnimationTree"] }
        get_parameter! { animation_state: AnimationPlayback = animation_tree["playback"] }

        self.velocity = Vector2::zero();
        animation_state.travel("Attack");
    }

    #[inline]
    fn move_state(&mut self, owner: &KinematicBody2D, delta: f32) {
        child_node! { animation_tree: AnimationTree = owner["AnimationTree"] }
        get_parameter! { animation_state: AnimationPlayback = animation_tree["playback"] }

        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        input_vector.x = (input.get_action_strength("ui_right") -
            input.get_action_strength("ui_left")) as f32;

        input_vector.y = (input.get_action_strength("ui_down") -
            input.get_action_strength("ui_up")) as f32;

        if input_vector != Vector2::zero() {
            // in the video, the function "normalized" is used, which handles zero condition.
            // godot-rust does not have that function, instead there is a try_normalize.
            // since we only use the input_vector when it's none zero, I opted to use the
            // "normalize" function after the check for zero.
            input_vector = input_vector.normalize();

            animation_tree.set("parameters/Idle/blend_position", input_vector);
            animation_tree.set("parameters/Run/blend_position", input_vector);
            animation_tree.set("parameters/Attack/blend_position", input_vector);
            animation_state.travel("Run");

            self.velocity = self.velocity.move_towards(input_vector * MAX_SPEED, ACCELERATION * delta);
        } else {
            animation_state.travel("Idle");
            self.velocity = self.velocity.move_towards(Vector2::zero(), FRICTION * delta);
        }

        // FRAC_PI_4 was suggested by c-lion ide as an approximate constant of the
        // documented default value of 0.785398 for "floor_max_angle"

        self.velocity = owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);

        if input.is_action_just_pressed("ui_attack") {
            self.state = PlayerState::Attack
        }
    }
}
