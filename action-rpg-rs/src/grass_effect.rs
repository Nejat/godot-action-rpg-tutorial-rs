use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GrassEffect;

impl GrassEffect {
    fn new(_owner: &Node2D) -> Self {
        GrassEffect
    }
}

#[methods]
impl GrassEffect {
    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        child_node! { owner, "AnimatedSprite" => animated_sprite: AnimatedSprite }

        animated_sprite.set_frame(0);
        animated_sprite.play("Animate", false);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_AnimatedSprite_animation_finished(&mut self, owner: &Node2D) {
        owner.queue_free();
    }
}