use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct SoftCollision {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for SoftCollision {
    fn init(base: Base<Area2D>) -> Self {
        SoftCollision { base }
    }
}

#[godot_api]
impl SoftCollision {
    #[func]
    pub fn get_push_vector(&self) -> Vector2 {
        let areas = self.base().get_overlapping_areas();
        let mut push_vector = Vector2::ZERO;

        if !areas.is_empty() {
            let area: Gd<Node2D> = areas.at(0).upcast();

            push_vector = area
                .get_global_position()
                .direction_to(self.base().get_global_position());

            if push_vector != Vector2::ZERO {
                push_vector = push_vector.normalized();
            }
        }

        push_vector
    }

    #[func]
    pub fn is_colliding(&self) -> bool {
        !self.base().get_overlapping_areas().is_empty()
    }
}
