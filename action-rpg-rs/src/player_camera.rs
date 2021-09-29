use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;

#[derive(NativeClass)]
#[inherit(Camera2D)]
pub struct PlayerCamera;

impl PlayerCamera {
    fn new(_owner: &Camera2D) -> Self {
        PlayerCamera
    }
}

#[methods]
impl PlayerCamera {
    #[export]
    fn _ready(&mut self, owner: &Camera2D) {
        let top_left = child_node!(owner["Limits/TopLeft"]: Position2D).position();

        owner.set("limit_top", top_left.y);
        owner.set("limit_left", top_left.x);

        let bottom_right = child_node!(owner["Limits/BottomRight"]: Position2D).position();

        owner.set("limit_bottom", bottom_right.y);
        owner.set("limit_right", bottom_right.x);
    }
}