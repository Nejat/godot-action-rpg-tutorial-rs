use gdnative::prelude::*;

mod grass;
mod player;

mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<grass::Grass>();
}

godot_init!(init);