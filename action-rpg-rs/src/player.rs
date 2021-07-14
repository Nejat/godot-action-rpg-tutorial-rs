use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
}

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
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        input_vector.x = (input.get_action_strength("ui_right") -
            input.get_action_strength("ui_left")) as f32;

        input_vector.y = (input.get_action_strength("ui_down") -
            input.get_action_strength("ui_up")) as f32;

        if input_vector != Vector2::zero() {
            self.velocity = input_vector;
        } else {
            self.velocity = Vector2::zero()
        }

        match owner.move_and_collide(self.velocity, true, true, false) {
            Some(_collision) => {}
            None => {}
        }
    }
}
