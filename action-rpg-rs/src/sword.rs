use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

pub(crate) const PROPERTY_DAMAGE: &str = "damage";
pub(crate) const PROPERTY_KNOCK_BACK_VECTOR: &str = "knock_back_vector";

const DEFAULT_DAMAGE: i64 = 1;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Sword {
    base: Base<Area2D>,
    #[var]
    pub damage: i64,
    #[var]
    pub knock_back_vector: Vector2,
}

#[godot_api]
impl IArea2D for Sword {
    fn init(base: Base<Area2D>) -> Self {
        Sword {
            base,
            damage: DEFAULT_DAMAGE,
            knock_back_vector: Vector2::ZERO,
        }
    }
}
