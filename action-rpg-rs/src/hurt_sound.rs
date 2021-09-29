use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(AudioStreamPlayer)]
pub struct PlayerHurtSound;

impl PlayerHurtSound {
    fn new(_owner: &AudioStreamPlayer) -> Self {
        PlayerHurtSound
    }
}

#[methods]
impl PlayerHurtSound {
    #[export]
    #[allow(non_snake_case)]
    fn _on_PlayerHurtSound_finished(&mut self, owner: &AudioStreamPlayer) {
        owner.queue_free();
    }
}