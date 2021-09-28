use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;

pub(crate) const PROPERTY_WANDER_RANGE: &str = "wander_ranger";

const DEFAULT_WANDER_RANGE: i64 = 32;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register)]
pub struct WanderController {
    rand: Ref<RandomNumberGenerator, Unique>,
    start_position: Vector2,
    target_position: Vector2,
    timer: Option<Ref<Timer>>,
    #[property]
    wander_ranger: i64,
}

impl WanderController {
    fn new(_owner: &Node2D) -> Self {
        WanderController {
            rand: RandomNumberGenerator::new(),
            start_position: Vector2::zero(),
            target_position: Vector2::zero(),
            timer: None,
            wander_ranger: DEFAULT_WANDER_RANGE
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<i64>(PROPERTY_WANDER_RANGE)
            .with_default(DEFAULT_WANDER_RANGE)
            .done();
    }
}

#[methods]
impl WanderController {
    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.start_position = owner.global_position();
        self.target_position = owner.global_position();
        self.timer = Some(child_node!(claim owner["Timer"]: Timer));
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_Timer_timeout(&mut self, _owner: &Node2D) {
        let target_vector = Vector2::new(self.random_wander_range(), self.random_wander_range());

        self.target_position = self.start_position + target_vector
    }

    fn random_wander_range(&self) -> f32 {
        self.rand.randf_range(-self.wander_ranger as f64, self.wander_ranger as f64) as f32
    }
}