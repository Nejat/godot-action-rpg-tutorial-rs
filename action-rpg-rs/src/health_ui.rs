use godot::classes::{Control, IControl, TextureRect};
use godot::prelude::*;

use crate::stats::{PROPERTY_MAX_HEALTH, SIGNAL_HEALTH_CHANGED, SIGNAL_MAX_HEALTH_CHANGED};

type Hearts = i64;

const DEFAULT_MAX_HEARTS: Hearts = 4;

const MINIMUM_HEARTS: Hearts = 0;
const MINIMUM_MAX_HEARTS: Hearts = 1;

const HEART_HEIGHT: f32 = 11.0;
const HEART_WIDTH: f32 = 15.0;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct HealthUI {
    base: Base<Control>,
    hearts: Hearts,
    hearts_empty: Option<Gd<TextureRect>>,
    hearts_full: Option<Gd<TextureRect>>,
    player_stats: Option<Gd<Node>>,
    max_hearts: Hearts,
}

#[godot_api]
impl IControl for HealthUI {
    fn init(base: Base<Control>) -> Self {
        HealthUI {
            base,
            hearts: DEFAULT_MAX_HEARTS,
            hearts_empty: None,
            hearts_full: None,
            player_stats: None,
            max_hearts: DEFAULT_MAX_HEARTS,
        }
    }

    fn ready(&mut self) {
        let hearts_empty = self.base().get_node_as::<TextureRect>("HeartUIEmpty");

        self.hearts_empty = Some(hearts_empty);

        let hearts_full = self.base().get_node_as::<TextureRect>("HeartUIFull");

        self.hearts_full = Some(hearts_full);

        let mut player_stats = self.base().get_node_as::<Node>("/root/PlayerStats");

        let callable_hearts = self.base().callable("set_hearts");

        player_stats.connect(SIGNAL_HEALTH_CHANGED, &callable_hearts);

        let callable_max = self.base().callable("set_max_hearts");

        player_stats.connect(SIGNAL_MAX_HEALTH_CHANGED, &callable_max);

        let max_health = player_stats
            .get(PROPERTY_MAX_HEALTH)
            .try_to::<i64>()
            .unwrap_or(0);

        self.internal_set_max_hearts(max_health);

        self.player_stats = Some(player_stats);
    }
}

#[godot_api]
impl HealthUI {
    #[func]
    pub fn set_hearts(&mut self, hearts: Hearts) {
        self.hearts = Hearts::clamp(hearts, MINIMUM_HEARTS, self.max_hearts);
        self.update_health_ui();
    }

    #[func]
    pub fn set_max_hearts(&mut self, max_hearts: Hearts) {
        self.internal_set_max_hearts(max_hearts);
    }

    fn internal_set_max_hearts(&mut self, max_hearts: Hearts) {
        self.max_hearts = Hearts::max(max_hearts, MINIMUM_MAX_HEARTS);

        self.update_max_health_ui();

        self.hearts = self.max_hearts;

        self.update_health_ui();
    }

    #[inline]
    fn update_health_ui(&mut self) {
        if let Some(full) = self.hearts_full.as_mut() {
            full.set_size(Vector2::new(self.hearts as f32 * HEART_WIDTH, HEART_HEIGHT));
        }
    }

    #[inline]
    fn update_max_health_ui(&mut self) {
        if let Some(empty) = self.hearts_empty.as_mut() {
            empty.set_size(Vector2::new(
                self.max_hearts as f32 * HEART_WIDTH,
                HEART_HEIGHT,
            ));
        }
    }
}
