use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct PlayerDetectionZone;

impl PlayerDetectionZone {
    fn new(_owner: &Area2D) -> Self {
        PlayerDetectionZone
    }
}

#[methods]
impl PlayerDetectionZone {}