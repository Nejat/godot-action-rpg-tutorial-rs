use godot::classes::{Area2D, CollisionShape2D, IArea2D, PackedScene, Timer};
use godot::prelude::*;

use crate::has_effect::HasEffect;

type Duration = f64;

const DEFAULT_INVINCIBLE: bool = false;

pub(crate) const SIGNAL_INVINCIBILITY_ENDED: &str = "invincibility_ended";
pub(crate) const SIGNAL_INVINCIBILITY_STARTED: &str = "invincibility_started";

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct HurtBox {
    base: Base<Area2D>,
    collision_shape: Option<Gd<CollisionShape2D>>,
    effect: Option<Gd<PackedScene>>,
    #[var]
    pub invincible: bool,
    timer: Option<Gd<Timer>>,
}

impl HasEffect for HurtBox {
    fn effect_scene(&self) -> &Option<Gd<PackedScene>> {
        &self.effect
    }
}

#[godot_api]
impl IArea2D for HurtBox {
    fn init(base: Base<Area2D>) -> Self {
        HurtBox {
            base,
            collision_shape: None,
            effect: None,
            invincible: DEFAULT_INVINCIBLE,
            timer: None,
        }
    }

    fn ready(&mut self) {
        self.effect = Some(load::<PackedScene>("res://Effects/HitEffect.tscn"));

        let collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        self.collision_shape = Some(collision_shape);

        let timer = self.base().get_node_as::<Timer>("Timer");

        self.timer = Some(timer);
    }
}

#[godot_api]
impl HurtBox {
    #[signal]
    fn invincibility_ended();

    #[signal]
    fn invincibility_started();

    #[func]
    pub fn play_hit_effect(&self) {
        let owner_node: Gd<Node2D> = self.base().clone().upcast();

        self.play_effect_root(&owner_node);
    }

    #[func]
    pub fn start_invincibility(&mut self, duration: Duration) {
        self.set_invincible_value(true);

        if let Some(timer) = self.timer.as_mut() {
            timer.start_ex().time_sec(duration).done();
        }
    }

    fn set_invincible_value(&mut self, invincible: bool) {
        self.invincible = invincible;

        if invincible {
            self.base_mut()
                .emit_signal(SIGNAL_INVINCIBILITY_STARTED, &[]);
        } else {
            self.base_mut().emit_signal(SIGNAL_INVINCIBILITY_ENDED, &[]);
        }
    }

    #[func]
    fn _on_hurt_box_invincibility_ended(&mut self) {
        if let Some(shape) = self.collision_shape.as_mut() {
            shape.set_disabled(false);
        }

        self.base_mut().set_monitorable(true);
    }

    #[func]
    fn _on_hurt_box_invincibility_started(&mut self) {
        if let Some(shape) = self.collision_shape.as_mut() {
            shape.set_disabled(true);
        }

        self.base_mut().set_monitorable(false);
    }

    #[func]
    fn _on_timer_timeout(&mut self) {
        self.set_invincible_value(false);
    }
}
