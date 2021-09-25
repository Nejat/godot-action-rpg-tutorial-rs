use gdnative::api::*;
use gdnative::prelude::*;

pub(crate) const DAMAGE: &'static str = "damage";
pub(crate) const KNOCK_BACK_VECTOR: &'static str = "knock_back_vector";

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Sword {
    #[property(default = 1)]
    damage: i64,
    #[property]
    knock_back_vector: Vector2,
}

impl Sword {
    pub(super) fn new(_owner: &Area2D) -> Self {
        Sword {
            damage: 1,
            knock_back_vector: Vector2::zero(),
        }
    }
}

#[methods]
impl Sword {}
