use godot::classes::{AudioStreamPlayer, IAudioStreamPlayer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AudioStreamPlayer)]
pub struct PlayerHurtSound {
    base: Base<AudioStreamPlayer>,
}

#[godot_api]
impl IAudioStreamPlayer for PlayerHurtSound {
    fn init(base: Base<AudioStreamPlayer>) -> Self {
        PlayerHurtSound { base }
    }
}

#[godot_api]
impl PlayerHurtSound {
    #[func]
    fn _on_player_hurt_sound_finished(&mut self) {
        self.base_mut().queue_free();
    }
}
