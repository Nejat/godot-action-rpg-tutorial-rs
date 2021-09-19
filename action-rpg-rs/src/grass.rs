use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass;

impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Grass
    }
}

#[methods]
impl Grass {
    #[export]
    fn _process(&mut self, owner: &Node2D, _delta: f32) {
        let input = Input::godot_singleton();

        if input.is_action_just_pressed("ui_attack") {
            owner.queue_free();
        }
    }
}