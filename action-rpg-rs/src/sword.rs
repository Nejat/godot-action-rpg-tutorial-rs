use gdnative::api::*;
use gdnative::prelude::*;

pub(crate) const KNOCK_BACK_VECTOR: &'static str = "knock_back_vector";

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Sword {
    #[property]
    knock_back_vector: Vector2
}

impl Sword {
    pub(super) fn new(_owner: &Area2D) -> Self {
        Sword {
            knock_back_vector: Vector2::zero()
        }
    }
}

#[methods]
impl Sword {}
