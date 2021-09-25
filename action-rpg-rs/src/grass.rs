use gdnative::api::*;
use gdnative::prelude::*;

use crate::has_effect::HasEffect;
use crate::load_resource;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass {
    effect: Option<Ref<PackedScene>>,
}

impl HasEffect for Grass {
    fn effect_scene(&self) -> &Option<Ref<PackedScene>> {
        &self.effect
    }
}

impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Grass { effect: None }
    }
}

#[methods]
impl Grass {
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        load_resource! { scene: PackedScene = "Effects/GrassEffect.tscn" {
            self.effect = Some(scene.claim())
        } }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, owner: &Node2D, _area: Ref<Area2D>) {
        self.play_effect(owner);
        owner.queue_free();
    }
}