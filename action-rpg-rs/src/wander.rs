use godot::classes::{INode2D, RandomNumberGenerator, Timer};
use godot::prelude::*;

type Duration = f64;
type WanderRange = i64;

const DEFAULT_WANDER_RANGE: WanderRange = 32;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct WanderController {
    base: Base<Node2D>,
    rand: Gd<RandomNumberGenerator>,
    start_position: Vector2,
    #[var]
    pub target_position: Vector2,
    timer: Option<Gd<Timer>>,
    #[var]
    pub wander_ranger: WanderRange,
}

#[godot_api]
impl INode2D for WanderController {
    fn init(base: Base<Node2D>) -> Self {
        WanderController {
            base,
            rand: RandomNumberGenerator::new_gd(),
            start_position: Vector2::ZERO,
            target_position: Vector2::ZERO,
            timer: None,
            wander_ranger: DEFAULT_WANDER_RANGE,
        }
    }

    fn ready(&mut self) {
        self.start_position = self.base().get_global_position();

        let timer = self.base().get_node_as::<Timer>("Timer");

        self.timer = Some(timer);

        // bats seemed to have the same pattern if they
        // were not interrupted by a chase state, this fixed that
        self.rand.randomize();
        self.update_target_position();
    }
}

#[godot_api]
impl WanderController {
    #[func]
    pub fn is_timer_complete(&self) -> bool {
        self.timer
            .as_ref()
            .map(|t| t.get_time_left() == 0.0)
            .unwrap_or(true)
    }

    #[func]
    pub fn start_timer(&mut self, duration: Duration) {
        if let Some(timer) = self.timer.as_mut() {
            timer.start_ex().time_sec(duration).done();
        }
    }

    #[func]
    fn _on_timer_timeout(&mut self) {
        self.update_target_position();
    }

    #[inline]
    fn random_wander_range(&mut self) -> f32 {
        self.rand
            .randf_range(-self.wander_ranger as f32, self.wander_ranger as f32)
    }

    #[inline]
    fn update_target_position(&mut self) {
        let target_vector = Vector2::new(self.random_wander_range(), self.random_wander_range());

        self.target_position = self.start_position + target_vector;
    }
}
