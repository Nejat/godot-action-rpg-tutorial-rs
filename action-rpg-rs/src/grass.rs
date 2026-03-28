use godot::classes::{Area2D, INode2D, PackedScene};
use godot::prelude::*;

use crate::has_effect::HasEffect;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Grass {
    base: Base<Node2D>,
    effect: Option<Gd<PackedScene>>,
}

impl HasEffect for Grass {
    fn effect_scene(&self) -> &Option<Gd<PackedScene>> {
        &self.effect
    }
}

#[godot_api]
impl INode2D for Grass {
    fn init(base: Base<Node2D>) -> Self {
        Grass { base, effect: None }
    }

    fn ready(&mut self) {
        self.effect = Some(load::<PackedScene>("res://Effects/GrassEffect.tscn"));
    }
}

#[godot_api]
impl Grass {
    #[func]
    fn _on_hurt_box_area_entered(&mut self, _area: Gd<Area2D>) {
        let owner: Gd<Node2D> = self.base().clone().upcast();

        self.play_effect_parent(&owner);
        self.base_mut().queue_free();
    }
}
