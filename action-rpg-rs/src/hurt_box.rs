use gdnative::api::*;
use gdnative::prelude::*;

use crate::has_effect::HasEffect;
use crate::load_resource;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct HurtBox {
    effect: Option<Ref<PackedScene>>,
    #[property(default = true)]
    show_hit: bool,
}

impl HasEffect for HurtBox {
    fn effect_scene(&self) -> &Option<Ref<PackedScene>> {
        &self.effect
    }
}

impl HurtBox {
    fn new(_owner: &Node2D) -> Self {
        HurtBox {
            effect: None,
            show_hit: true
        }
    }
}

#[methods]
impl HurtBox {
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        load_resource! { scene: PackedScene = "Effects/HitEffect.tscn" {
            self.effect = Some(scene.claim())
        } }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&self, owner: &Node2D, _area: Ref<Area2D>) {
        if self.show_hit {
            self.play_effect_root(owner);
        }
    }
}