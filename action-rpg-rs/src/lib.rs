use gdnative::prelude::*;

mod player;
mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
}

godot_init!(init);