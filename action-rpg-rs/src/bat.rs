use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::get_parameter;

const FRICTION: f32 = 200.0;
const KNOCK_BACK_FORCE: f32 = 120.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat {
    knock_back: Vector2,
}

impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            knock_back: Vector2::zero()
        }
    }
}

#[methods]
impl Bat {
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        self.knock_back = self.knock_back.move_towards(Vector2::zero(), FRICTION * delta);
        self.knock_back = owner.move_and_slide(self.knock_back, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        self.knock_back = get_parameter!{ area["knock_back_vector"] }.to_vector2() * KNOCK_BACK_FORCE;
    }
}