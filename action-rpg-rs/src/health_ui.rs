use gdnative::api::*;
use gdnative::prelude::*;

use crate::stats::{PROPERTY_MAX_HEALTH, SIGNAL_HEALTH_CHANGED, SIGNAL_MAX_HEALTH_CHANGED};
use crate::{assume_safe, auto_load, child_node};

type Hearts = i64;

pub(crate) const PROPERTY_HEARTS: &str = "hearts";
pub(crate) const PROPERTY_MAX_HEARTS: &str = "max_hearts";

const DEFAULT_MAX_HEARTS: Hearts = 4;

const MINIMUM_HEARTS: Hearts = 0;
const MINIMUM_MAX_HEARTS: Hearts = 1;

const HEART_HEIGHT: f32 = 11.0;
const HEART_WIDTH: f32 = 15.0;

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register)]
pub struct HealthUI {
    #[property]
    hearts: Hearts,
    hearts_empty: Option<Ref<TextureRect>>,
    hearts_full: Option<Ref<TextureRect>>,
    player_stats: Option<Ref<Node>>,
    #[property]
    max_hearts: Hearts,
}

impl HealthUI {
    fn new(_owner: TRef<Control>) -> Self {
        HealthUI {
            hearts: DEFAULT_MAX_HEARTS,
            hearts_empty: None,
            hearts_full: None,
            player_stats: None,
            max_hearts: DEFAULT_MAX_HEARTS,
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .property::<Hearts>(PROPERTY_HEARTS)
            .with_getter(|s: &Self, _| s.hearts)
            .with_setter(Self::set_hearts)
            .with_default(DEFAULT_MAX_HEARTS)
            .done();

        builder
            .property::<Hearts>(PROPERTY_MAX_HEARTS)
            .with_getter(|s: &Self, _| s.max_hearts)
            .with_setter(Self::set_max_hearts)
            .with_default(DEFAULT_MAX_HEARTS)
            .done();
    }

    #[inline]
    fn update_health_ui(&self) {
        assume_safe!(self.hearts_full).set_size(
            Vector2::new(self.hearts as f32 * HEART_WIDTH, HEART_HEIGHT),
            false,
        );
    }

    #[inline]
    fn update_max_health_ui(&self) {
        assume_safe!(self.hearts_empty).set_size(
            Vector2::new(self.max_hearts as f32 * HEART_WIDTH, HEART_HEIGHT),
            false,
        );
    }
}

#[methods]
impl HealthUI {
    #[method]
    fn _ready(&mut self, #[base] owner: TRef<Control>) {
        let owner_ref = owner;

        self.hearts_empty = Some(child_node!(claim owner_ref["HeartUIEmpty"]: TextureRect));
        self.hearts_full = Some(child_node!(claim owner_ref["HeartUIFull"]: TextureRect));

        let player_stats = auto_load!("PlayerStats": Node);

        player_stats
            .connect(
                SIGNAL_HEALTH_CHANGED,
                owner,
                "set_hearts",
                VariantArray::new_shared(),
                1,
            )
            .expect("set_hearts to connect to player stats");

        player_stats
            .connect(
                SIGNAL_MAX_HEALTH_CHANGED,
                owner,
                "set_max_hearts",
                VariantArray::new_shared(),
                1,
            )
            .expect("set_max_hearts to connect to player stats");

        self.set_max_hearts(
            owner,
            player_stats
                .get(PROPERTY_MAX_HEALTH)
                .try_to::<i64>()
                .unwrap_or(0),
        );

        self.player_stats = Some(player_stats.claim());
    }

    #[method]
    fn set_hearts(&mut self, #[base] _owner: TRef<Control>, hearts: Hearts) {
        self.hearts = Hearts::clamp(hearts, MINIMUM_HEARTS, self.max_hearts);
        self.update_health_ui();
    }

    #[method]
    fn set_max_hearts(&mut self, #[base] _owner: TRef<Control>, max_hearts: Hearts) {
        self.max_hearts = Hearts::max(max_hearts, MINIMUM_MAX_HEARTS);
        self.update_max_health_ui();

        self.hearts = self.max_hearts;
        self.update_health_ui();
    }
}
