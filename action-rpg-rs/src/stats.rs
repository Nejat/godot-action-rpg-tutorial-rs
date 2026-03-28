use godot::classes::INode;
use godot::prelude::*;

pub(crate) const PROPERTY_HEALTH: &str = "health";
pub(crate) const PROPERTY_MAX_HEALTH: &str = "max_health";

#[allow(
    dead_code,
    reason = "used by set_health_value which is reserved for future signal-based API"
)]
const MINIMUM_HEALTH: Health = 0;
#[allow(
    dead_code,
    reason = "used by set_max_health_value which is reserved for future signal-based API"
)]
const MINIMUM_MAX_HEALTH: Health = 1;

pub(crate) const SIGNAL_HEALTH_CHANGED: &str = "health_changed";
pub(crate) const SIGNAL_MAX_HEALTH_CHANGED: &str = "max_health_changed";
#[allow(
    dead_code,
    reason = "used by set_health_value which is reserved for future signal-based API"
)]
pub(crate) const SIGNAL_NO_HEALTH: &str = "no_health";

type Health = i64;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Stats {
    base: Base<Node>,
    #[var]
    pub health: Health,
    #[var]
    pub max_health: Health,
}

#[godot_api]
impl INode for Stats {
    fn init(base: Base<Node>) -> Self {
        Stats {
            base,
            health: Health::default(),
            max_health: Health::default(),
        }
    }
}

#[godot_api]
impl Stats {
    #[signal]
    fn health_changed(health: i64);

    #[signal]
    fn max_health_changed(max_health: i64);

    #[signal]
    fn no_health();

    #[allow(dead_code, reason = "reserved for future signal-based API")]
    pub fn set_max_health_value(&mut self, max_health: Health) {
        self.max_health = Health::max(max_health, MINIMUM_MAX_HEALTH);

        let val = self.max_health.to_variant();

        self.base_mut()
            .emit_signal(SIGNAL_MAX_HEALTH_CHANGED, &[val]);

        let new_health = self.max_health;

        self.set_health_value(new_health);
    }

    #[allow(dead_code, reason = "reserved for future signal-based API")]
    pub fn set_health_value(&mut self, health: Health) {
        self.health = Health::clamp(health, MINIMUM_HEALTH, self.max_health);

        let val = self.health.to_variant();

        self.base_mut().emit_signal(SIGNAL_HEALTH_CHANGED, &[val]);

        if self.health <= MINIMUM_HEALTH {
            self.base_mut().emit_signal(SIGNAL_NO_HEALTH, &[]);
        }
    }
}
