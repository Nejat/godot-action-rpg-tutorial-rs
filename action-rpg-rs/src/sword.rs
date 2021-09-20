use gdnative::api::*;
use gdnative::prelude::*;
use gdnative::private::godot_object::Sealed;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Sword {
    #[property]
    knock_back_vector: Vector2
}

impl Sealed for Sword {}

unsafe impl GodotObject for Sword {
    type RefKind = RefCounted;

    fn class_name() -> &'static str {
        stringify!(Sword)
    }
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
