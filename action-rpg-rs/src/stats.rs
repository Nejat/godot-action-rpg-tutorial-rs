use gdnative::api::*;
use gdnative::prelude::*;

pub(crate) const PROPERTY_HEALTH: &'static str = "health";
pub(crate) const PROPERTY_MAX_HEALTH: &'static str = "max_health";

const DEFAULT_HEALTH: Health = 1;

pub(crate) const SIGNAL_NO_HEALTH: &str = "no_health";

type Health = i64;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Stats {
    health: Health,
    max_health: Health,
}

impl Stats {
    fn new(_owner: &Node) -> Self {
        Stats {
            health: Health::default(),
            max_health: Health::default(),
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<Health>(PROPERTY_MAX_HEALTH)
            .with_getter(|s: &Self, _| s.max_health)
            .with_setter(Self::set_max_health)
            .with_default(DEFAULT_HEALTH)
            .done();

        builder
            .add_property::<Health>(PROPERTY_HEALTH)
            .with_getter(|s: &Self, _| s.health)
            .with_setter(Self::set_health)
            .with_default(DEFAULT_HEALTH)
            .done();

        builder.add_signal(Signal { name: SIGNAL_NO_HEALTH, args: &[] });
    }
}

#[methods]
impl Stats {
    fn set_max_health(&mut self, _owner: TRef<Node>, value: Health) {
        self.max_health = value;
        self.health = value;
    }

    fn set_health(&mut self, owner: TRef<Node>, value: Health) {
        self.health = value;
        if value <= 0 {
            owner.emit_signal(SIGNAL_NO_HEALTH, &[]);
        }
    }
}