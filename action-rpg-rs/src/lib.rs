use gdnative::prelude::*;

mod bat;
mod grass;
mod grass_effect;
mod player;
mod sword;

mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<bat::Bat>();
    handle.add_class::<player::Player>();
    handle.add_class::<grass::Grass>();
    handle.add_class::<grass_effect::GrassEffect>();
    handle.add_class::<sword::Sword>();
}

godot_init!(init);