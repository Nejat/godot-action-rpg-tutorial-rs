use godot::classes::{Camera2D, ICamera2D, Marker2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct PlayerCamera {
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for PlayerCamera {
    fn init(base: Base<Camera2D>) -> Self {
        PlayerCamera { base }
    }

    fn ready(&mut self) {
        let top_left = self.base().get_node_as::<Marker2D>("Limits/TopLeft");
        let top_left_pos = top_left.get_position();

        self.base_mut()
            .set("limit_top", &(top_left_pos.y as i32).to_variant());

        self.base_mut()
            .set("limit_left", &(top_left_pos.x as i32).to_variant());

        let bottom_right = self.base().get_node_as::<Marker2D>("Limits/BottomRight");
        let bottom_right_pos = bottom_right.get_position();

        self.base_mut()
            .set("limit_bottom", &(bottom_right_pos.y as i32).to_variant());

        self.base_mut()
            .set("limit_right", &(bottom_right_pos.x as i32).to_variant());
    }
}
