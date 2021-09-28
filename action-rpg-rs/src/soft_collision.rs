use gdnative::api::*;
use gdnative::prelude::*;

use crate::array_item;

pub(crate) const METHOD_IS_COLLIDING: &str = "is_colliding";
pub(crate) const METHOD_GET_PUSH_VECTOR: &str = "get_push_vector";

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct SoftCollision;

impl SoftCollision {
    fn new(_owner: &Area2D) -> Self { SoftCollision }
}

#[methods]
impl SoftCollision {
    #[export]
    fn get_push_vector (&self, owner: &Area2D) -> Vector2 {
        let mut areas = owner.get_overlapping_areas();
        let mut push_vector = Vector2::zero();

        if !areas.is_empty() {
            let area = array_item! { areas[0]: Node2D };

            push_vector = area.global_position().direction_to(owner.global_position());

            if push_vector != Vector2::zero() {
                push_vector = push_vector.normalize();
            }
        }

        push_vector
    }

    #[export]
    fn is_colliding (&self, owner: &Area2D) -> bool {
        !owner.get_overlapping_areas().is_empty()
    }
}