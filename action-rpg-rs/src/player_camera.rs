use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;

#[derive(NativeClass)]
#[inherit(Camera2D)]
pub struct PlayerCamera;

impl PlayerCamera {
    fn new(_owner: TRef<Camera2D>) -> Self {
        PlayerCamera
    }
}

#[methods]
impl PlayerCamera {
    #[method]
    fn _ready(&mut self, #[base] owner: TRef<Camera2D>) {
        let owner_ref = owner;
        let top_left = child_node!(owner_ref["Limits/TopLeft"]: Position2D).position();

        owner_ref.set("limit_top", top_left.y);
        owner_ref.set("limit_left", top_left.x);

        let bottom_right = child_node!(owner_ref["Limits/BottomRight"]: Position2D).position();

        owner_ref.set("limit_bottom", bottom_right.y);
        owner_ref.set("limit_right", bottom_right.x);
    }
}
