use gdnative::api::*;
use gdnative::prelude::*;

use crate::{auto_load, child_node, set_parameter};
use crate::stats::{PROPERTY_MAX_HEALTH, SIGNAL_HEALTH_CHANGED};

type Hearts = i64;

pub(crate) const PROPERTY_HEARTS: &str = "hearts";
pub(crate) const PROPERTY_MAX_HEARTS: &str = "max_hearts";

const DEFAULT_MAX_HEARTS: Hearts = 4;

const MINIMUM_HEARTS: Hearts = 0;
const MINIMUM_MAX_HEARTS: Hearts = 1;


#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register)]
pub struct HealthUI {
    #[property]
    hearts: Hearts,
    label: Option<Ref<Label>>,
    player_stats: Option<Ref<Node>>,
    #[property]
    max_hearts: Hearts,
}

impl HealthUI {
    fn new(_owner: &Control) -> Self {
        HealthUI {
            hearts: DEFAULT_MAX_HEARTS,
            label: None,
            player_stats: None,
            max_hearts: DEFAULT_MAX_HEARTS,
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<Hearts>(PROPERTY_HEARTS)
            .with_getter(|s: &Self, _| s.hearts)
            .with_setter(Self::set_hearts)
            .with_default(DEFAULT_MAX_HEARTS)
            .done();

        builder
            .add_property::<Hearts>(PROPERTY_MAX_HEARTS)
            .with_getter(|s: &Self, _| s.max_hearts)
            .with_setter(Self::set_max_hearts)
            .with_default(DEFAULT_MAX_HEARTS)
            .done();
    }
}

#[methods]
impl HealthUI {
    #[export]
    fn _ready(&mut self, owner: TRef<Control>) {
        let owner_ref = owner.as_ref();

        self.label = Some(child_node!(claim owner_ref["Label"]: Label));

        let player_stats = auto_load!("PlayerStats": Node);

        player_stats
            .connect(SIGNAL_HEALTH_CHANGED, owner.clone(), "set_hearts", VariantArray::new_shared(), 1)
            .expect("set_hearts to connect to player stats");

        self.set_max_hearts(owner, player_stats.get(PROPERTY_MAX_HEALTH).to_i64());

        self.player_stats = Some(player_stats.claim());
    }

    #[export]
    fn set_hearts(&mut self, _owner: TRef<Control>, hearts: Hearts) {
        self.hearts = Hearts::clamp(hearts, MINIMUM_HEARTS, self.max_hearts);

        set_parameter! { ?self.label; "text" = format!("HP = {}", self.hearts) }
    }

    #[export]
    fn set_max_hearts(&mut self, _owner: TRef<Control>, max_hearts: Hearts) {
        self.max_hearts = Hearts::max(max_hearts, MINIMUM_MAX_HEARTS);
        self.hearts = self.max_hearts;
    }
}