use gdnative::api::*;
use gdnative::prelude::*;

use crate::has_effect::HasEffect;
use crate::load_resource;

pub(crate) const PROPERTY_SHOW_HIT: &str = "show_hit";

const DEFAULT_SHOW_HIT: bool = true;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register)]
pub struct HurtBox {
    effect: Option<Ref<PackedScene>>,
    #[property]
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
            show_hit: DEFAULT_SHOW_HIT
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<bool>(PROPERTY_SHOW_HIT)
            .with_getter(|s: &Self, _| s.show_hit)
            .with_setter(|s: &mut Self, _, value: bool| s.show_hit = value)
            .with_default(DEFAULT_SHOW_HIT)
            .done();
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