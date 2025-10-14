use gdnative::api::*;
use gdnative::prelude::*;

use crate::array_item;

pub(crate) const METHOD_IS_COLLIDING: &str = "is_colliding";
pub(crate) const METHOD_GET_PUSH_VECTOR: &str = "get_push_vector";

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct SoftCollision;

impl SoftCollision {
    fn new(_owner: TRef<Area2D>) -> Self {
        SoftCollision
    }
}

#[methods]
impl SoftCollision {
    #[method]
    fn get_push_vector(&self, #[base] owner: TRef<Area2D>) -> Vector2 {
        let owner_ref = owner;
        let areas = owner_ref.get_overlapping_areas();
        let mut push_vector = Vector2::new(0.0, 0.0);

        if !areas.is_empty() {
            let area = array_item! { areas[0]: Node2D };

            push_vector = area.global_position().direction_to(owner_ref.global_position());

            if push_vector != Vector2::new(0.0, 0.0) {
                push_vector = push_vector.normalized();
            }
        }

        push_vector
    }

    #[method]
    fn is_colliding(&self, #[base] owner: TRef<Area2D>) -> bool {
        !owner.get_overlapping_areas().is_empty()
    }
}
