use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;
use crate::get_parameter;
use crate::set_parameter;
use crate::stats::PROPERTY_HEALTH;
use crate::sword::KNOCK_BACK_VECTOR;

const FRICTION: f32 = 200.0;
const KNOCK_BACK_FORCE: f32 = 120.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat {
    knock_back: Vector2,
    stats: Option<Ref<Node>>,
}

impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            knock_back: Vector2::zero(),
            stats: None,
        }
    }
}

#[methods]
impl Bat {
    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        child_node! { stats = owner["Stats"] }

        self.stats = Some(stats)
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        self.knock_back = self.knock_back.move_towards(Vector2::zero(), FRICTION * delta);
        self.knock_back = owner.move_and_slide(self.knock_back, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        let health = get_parameter!(self.stats.unwrap(); PROPERTY_HEALTH).to_i64() - 1;

        set_parameter!(self.stats.unwrap(); PROPERTY_HEALTH = health);
        self.knock_back = get_parameter!(area[KNOCK_BACK_VECTOR]).to_vector2() * KNOCK_BACK_FORCE;
    }

    // when connecting signal in the godot editor, click the "advanced" switch
    // and select the "deferred" option, otherwise an exception occurs
    // todo: figure out why this is necessary
    #[export]
    #[allow(non_snake_case)]
    fn _on_Stats_no_health(&self, owner: &KinematicBody2D) {
        owner.queue_free();
    }
}
