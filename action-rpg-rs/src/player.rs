use std::f64::consts::FRAC_PI_4;

use gdnative::prelude::*;

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
            velocity: Vector2::zero()
        }
    }
}

#[methods]
impl Player {
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        input_vector.x = (input.get_action_strength("ui_right") -
            input.get_action_strength("ui_left")) as f32;

        input_vector.y = (input.get_action_strength("ui_down") -
            input.get_action_strength("ui_up")) as f32;

        // in the video, the function "normalized" is used, which handles zero condition.
        // godot-rust does not have that function, instead there is a try_normalize.
        // since we only use the input_vector when it's none zero, I opted to use the
        // "normalize" function after the check for zero.

        if input_vector != Vector2::zero() {
            self.velocity = self.velocity.move_towards(input_vector.normalize() * MAX_SPEED, ACCELERATION * delta)
        } else {
            self.velocity = self.velocity.move_towards(Vector2::zero(), FRICTION * delta)
        }

        // the additional values passed to "move_and_slide" here, that are not mentioned in
        // the video, are listed in the api documentation and are probably defaults in gdscript
        
        // additionally FRAC_PI_4 was suggested by c-lion ide as an approximate constant of the
        // documented default value of 0.785398 for "floor_max_angle"

        self.velocity = owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }
}