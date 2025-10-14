use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(AudioStreamPlayer)]
pub struct PlayerHurtSound;

impl PlayerHurtSound {
    fn new(_owner: TRef<AudioStreamPlayer>) -> Self {
        PlayerHurtSound
    }
}

#[methods]
impl PlayerHurtSound {
    #[method]
    #[allow(non_snake_case)]
    fn _on_PlayerHurtSound_finished(&mut self, #[base] owner: TRef<AudioStreamPlayer>) {
        owner.queue_free();
    }
}
