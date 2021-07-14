use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::utils::get_node;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
}

const ACCELERATION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;
const FRICTION: f32 = 500.0;

impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
        }
    }
}

// the additional values passed to godot functions, that are not mentioned in
// the video, are listed in the api documentation and are defaults in gdscript

#[methods]
impl Player {
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        let animation_player = get_node::<AnimationPlayer>(owner, "AnimationPlayer");

        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        input_vector.x = (input.get_action_strength("ui_right") -
            input.get_action_strength("ui_left")) as f32;

        input_vector.y = (input.get_action_strength("ui_down") -
            input.get_action_strength("ui_up")) as f32;

        if input_vector != Vector2::zero() {
            if input_vector.x > 0.0 {
                animation_player.play("RunRight", -1.0, 1.0, false);
            } else {
                animation_player.play("RunLeft", -1.0, 1.0, false);
            }

            // in the video, the function "normalized" is used, which handles zero condition.
            // godot-rust does not have that function, instead there is a try_normalize.
            // since we only use the input_vector when it's none zero, I opted to use the
            // "normalize" function after the check for zero.

            self.velocity = self.velocity.move_towards(input_vector.normalize() * MAX_SPEED, ACCELERATION * delta);
        } else {
            self.velocity = self.velocity.move_towards(Vector2::zero(), FRICTION * delta);

            if self.velocity.x >= 0.0 {
                animation_player.play("IdleRight", -1.0, 1.0, false);
            } else {
                animation_player.play("IdleLeft", -1.0, 1.0, false);
            }
        }

        // FRAC_PI_4 was suggested by c-lion ide as an approximate constant of the
        // documented default value of 0.785398 for "floor_max_angle"

        self.velocity = owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }
}
