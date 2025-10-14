// Allow macro metavariables in unsafe blocks - these are utility macros designed to encapsulate
// unsafe Godot operations and the warnings are about macro design rather than actual unsafe behavior
#![allow(clippy::macro_metavars_in_unsafe)]
// Allow unexpected cfg condition from GDNative's godot_init! macro - this warning comes from
// the macro's internal cfg attributes and is not something we can control in our application code
#![allow(unexpected_cfgs)]

use gdnative::prelude::*;

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

mod utils;

fn init(handle: InitHandle) {
    handle.add_class::<health_ui::HealthUI>();
    handle.add_class::<player_detection::PlayerDetectionZone>();
    handle.add_class::<hurt_box::HurtBox>();
    handle.add_class::<stats::Stats>();
    handle.add_class::<sword::Sword>();
    handle.add_class::<soft_collision::SoftCollision>();
    handle.add_class::<wander::WanderController>();
    handle.add_class::<bat::Bat>();
    handle.add_class::<hurt_sound::PlayerHurtSound>();
    handle.add_class::<player_camera::PlayerCamera>();
    handle.add_class::<player::Player>();
    handle.add_class::<effect::Effect>();
    handle.add_class::<grass::Grass>();
}

godot_init!(init);
