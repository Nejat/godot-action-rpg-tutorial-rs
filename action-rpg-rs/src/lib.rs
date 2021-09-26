use gdnative::prelude::*;

mod bat;
mod effect;
mod grass;
mod has_effect;
mod hurt_box;
mod player;
mod player_detection;
mod stats;
mod sword;

mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<player_detection::PlayerDetectionZone>();
    handle.add_class::<hurt_box::HurtBox>();
    handle.add_class::<stats::Stats>();
    handle.add_class::<sword::Sword>();
    handle.add_class::<bat::Bat>();
    handle.add_class::<player::Player>();
    handle.add_class::<effect::Effect>();
    handle.add_class::<grass::Grass>();
}

godot_init!(init);