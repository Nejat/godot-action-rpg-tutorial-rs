use gdnative::api::*;
use gdnative::prelude::*;
use gdnative::nativescript::property::Usage;

pub(crate) const PROPERTY_DAMAGE: &str = "damage";
pub(crate) const PROPERTY_KNOCK_BACK_VECTOR: &str = "knock_back_vector";

const DEFAULT_DAMAGE: i64 = 1;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register)]
pub struct Sword {
    #[property]
    damage: i64,
    #[property]
    knock_back_vector: Vector2,
}

impl Sword {
    fn new(_owner: &Area2D) -> Self {
        Sword {
            damage: 1,
            knock_back_vector: Vector2::zero(),
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<i64>(PROPERTY_DAMAGE)
            .with_getter(|s: &Self, _| s.damage)
            .with_setter(|s: &mut Self, _, value: i64| s.damage = value)
            .with_default(DEFAULT_DAMAGE)
            .done();

        builder
            .add_property::<Vector2>(PROPERTY_KNOCK_BACK_VECTOR)
            .with_getter(|s: &Self, _| s.knock_back_vector)
            .with_setter(|s: &mut Self, _, value: Vector2| s.knock_back_vector = value)
            .with_usage(Usage::NOEDITOR)
            .done();
    }
}

#[methods]
impl Sword {}
