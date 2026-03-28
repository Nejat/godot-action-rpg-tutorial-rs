use godot::prelude::*;

mod bat;
mod effect;
mod grass;
mod has_effect;
mod health_ui;
mod hurt_box;
mod hurt_sound;
mod player;
mod player_camera;
mod player_detection;
mod soft_collision;
mod stats;
mod sword;
mod wander;

struct ActionRpgExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ActionRpgExtension {}
