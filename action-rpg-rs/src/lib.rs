use gdnative::prelude::*;

mod bat;
mod effect;
mod grass;
mod player;
mod stats;
mod sword;

mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<stats::Stats>();
    handle.add_class::<sword::Sword>();
    handle.add_class::<bat::Bat>();
    handle.add_class::<player::Player>();
    handle.add_class::<effect::Effect>();
    handle.add_class::<grass::Grass>();
}

godot_init!(init);