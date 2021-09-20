use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat;

impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat
    }
}

#[methods]
impl Bat {
    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, owner: &KinematicBody2D, _area: Ref<Area2D>) {
        owner.queue_free();
    }
}