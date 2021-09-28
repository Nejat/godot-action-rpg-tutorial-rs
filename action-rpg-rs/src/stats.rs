use gdnative::api::*;
use gdnative::prelude::*;

pub(crate) const PROPERTY_HEALTH: &'static str = "health";
pub(crate) const PROPERTY_MAX_HEALTH: &'static str = "max_health";

const DEFAULT_HEALTH: Health = 1;

const MINIMUM_HEALTH: Health = 0;
const MINIMUM_MAX_HEALTH: Health = 1;

pub(crate) const SIGNAL_HEALTH_CHANGED: &str = "health_changed";
pub(crate) const SIGNAL_MAX_HEALTH_CHANGED: &str = "max_health_changed";
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

        let health_arg = SignalArgument {
            name: "health",
            default: DEFAULT_HEALTH.to_variant(),
            export_info: ExportInfo::new(VariantType::I64),
            usage: PropertyUsage::DEFAULT
        };

        builder.add_signal(Signal { name: SIGNAL_HEALTH_CHANGED, args: &[health_arg] });

        let max_health_arg = SignalArgument {
            name: "max_health",
            default: DEFAULT_HEALTH.to_variant(),
            export_info: ExportInfo::new(VariantType::I64),
            usage: PropertyUsage::DEFAULT
        };

        builder.add_signal(Signal { name: SIGNAL_MAX_HEALTH_CHANGED, args: &[max_health_arg] });
        builder.add_signal(Signal { name: SIGNAL_NO_HEALTH, args: &[] });
    }
}

#[methods]
impl Stats {
    fn set_max_health(&mut self, owner: TRef<Node>, max_health: Health) {
        self.max_health = Health::max(max_health, MINIMUM_MAX_HEALTH);

        owner.emit_signal(SIGNAL_MAX_HEALTH_CHANGED, &[self.max_health.to_variant()]);

        self.set_health(owner, self.max_health);
    }

    fn set_health(&mut self, owner: TRef<Node>, health: Health) {
        self.health = Health::clamp(health, MINIMUM_HEALTH, self.max_health);

        owner.emit_signal(SIGNAL_HEALTH_CHANGED, &[self.health.to_variant()]);

        if health <= MINIMUM_HEALTH {
            owner.emit_signal(SIGNAL_NO_HEALTH, &[]);
        }
    }
}