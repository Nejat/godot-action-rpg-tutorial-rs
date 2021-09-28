use gdnative::api::*;
use gdnative::prelude::*;

use crate::{assume_safe, child_node, get_parameter};

type Duration = f64;
type WanderRange = i64;

pub(crate) const METHOD_IS_TIMER_COMPLETE: &str = "is_timer_complete";
pub(crate) const METHOD_START_TIMER: &str = "start_timer";

pub(crate) const PROPERTY_TARGET_POSITION: &str = "target_position";
pub(crate) const PROPERTY_WANDER_RANGE: &str = "wander_ranger";

const DEFAULT_WANDER_RANGE: WanderRange = 32;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register)]
pub struct WanderController {
    rand: Ref<RandomNumberGenerator, Unique>,
    start_position: Vector2,
    #[property]
    target_position: Vector2,
    timer: Option<Ref<Timer>>,
    #[property]
    wander_ranger: WanderRange,
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
            .with_getter(|s: &Self, _| s.wander_ranger)
            .with_setter(|s: &mut Self, _, value| s.wander_ranger = value)
            .with_default(DEFAULT_WANDER_RANGE)
            .done();

        builder
            .add_property::<Vector2>(PROPERTY_TARGET_POSITION)
            .with_getter(|s: &Self, _| s.target_position)
            .with_setter(|s: &mut Self, _, value| s.target_position = value)
            .with_usage(PropertyUsage::NOEDITOR)
            .done();
    }
}

#[methods]
impl WanderController {
    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.start_position = owner.global_position();
        self.timer = Some(child_node!(claim owner["Timer"]: Timer));

        // bats seemed to have to the same pattern if they
        // were not interrupted by a chase state, this fixed that
        self.rand.randomize();

        self.update_target_position();
    }

    #[export]
    fn is_timer_complete(&self, _owner: &Node2D) -> bool{
        get_parameter!(self.timer.unwrap(); "time_left").to_f64() == 0.0
    }

    #[export]
    fn start_timer(&self, _owner: &Node2D, duration: Duration) {
        assume_safe!(self.timer).start(duration);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_Timer_timeout(&mut self, _owner: &Node2D) {
        self.update_target_position()
    }

    #[inline]
    fn random_wander_range(&self) -> f32 {
        self.rand.randf_range(-self.wander_ranger as f64, self.wander_ranger as f64) as f32
    }

    #[inline]
    fn update_target_position(&mut self) {
        let target_vector = Vector2::new(self.random_wander_range(), self.random_wander_range());

        self.target_position = self.start_position + target_vector
    }
}