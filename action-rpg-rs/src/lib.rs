use gdnative::prelude::*;

mod bat;
mod grass;
mod grass_effect;
mod player;

mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<bat::Bat>();
    handle.add_class::<player::Player>();
    handle.add_class::<grass::Grass>();
    handle.add_class::<grass_effect::GrassEffect>();
}

godot_init!(init);