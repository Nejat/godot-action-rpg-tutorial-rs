use gdnative::api::*;
use gdnative::prelude::*;

use crate::{assume_safe, child_node, load_resource};
use crate::has_effect::HasEffect;

type Duration = f64;

pub(crate) const PROPERTY_INVINCIBLE: &str = "invincible";

const DEFAULT_INVINCIBLE: bool = false;

pub(crate) const METHOD_START_INVINCIBILITY: &str = "start_invincibility";
pub(crate) const METHOD_PLAY_HIT_EFFECT: &str = "play_hit_effect";

pub(crate) const SIGNAL_INVINCIBILITY_ENDED: &str = "invincibility_ended";
pub(crate) const SIGNAL_INVINCIBILITY_STARTED: &str = "invincibility_started";

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register)]
pub struct HurtBox {
    collision_shape: Option<Ref<CollisionShape2D>>,
    effect: Option<Ref<PackedScene>>,
    invincible: bool,
    timer: Option<Ref<Timer>>,
}

impl HasEffect for HurtBox {
    fn effect_scene(&self) -> &Option<Ref<PackedScene>> {
        &self.effect
    }
}

impl HurtBox {
    fn new(_owner: &Node2D) -> Self {
        HurtBox {
            collision_shape: None,
            effect: None,
            invincible: DEFAULT_INVINCIBLE,
            timer: None,
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<bool>(PROPERTY_INVINCIBLE)
            .with_getter(|s: &Self, _| s.invincible)
            .with_setter(Self::set_invincible)
            .with_default(DEFAULT_INVINCIBLE)
            .done();

        builder.add_signal(Signal { name: SIGNAL_INVINCIBILITY_ENDED, args: &[] });

        builder.add_signal(Signal { name: SIGNAL_INVINCIBILITY_STARTED, args: &[] });
    }

    fn set_invincible(&mut self, owner: TRef<Node2D>, invincible: bool) {
        self.invincible = invincible;

        if invincible {
            owner.emit_signal(SIGNAL_INVINCIBILITY_STARTED, &[]);
        } else {
            owner.emit_signal(SIGNAL_INVINCIBILITY_ENDED, &[]);
        }
    }
}

#[methods]
impl HurtBox {
    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        load_resource! { scene: PackedScene = "Effects/HitEffect.tscn" {
            self.effect = Some(scene.claim())
        } }

        self.collision_shape = Some(child_node!(claim owner["CollisionShape2D"]: CollisionShape2D));
        self.timer = Some(child_node!(claim owner["Timer"]: Timer));
    }

    #[export]
    fn play_hit_effect(&mut self, owner: &Node2D) {
        self.play_effect_root(owner);
    }

    #[export]
    fn start_invincibility(&mut self, owner: TRef<Node2D>, duration: Duration) {
        self.set_invincible(owner, true);

        assume_safe!(self.timer).start(duration);
    }

    // rust required these two signals to be connected "deferred" in godot
    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_invincibility_ended(&mut self, owner: &Node2D) {
        assume_safe!(self.collision_shape).set_disabled(false);
        owner.set("monitorable", true);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_invincibility_started(&mut self, owner: &Node2D) {
        assume_safe!(self.collision_shape).set_disabled(true);
        owner.set("monitorable", false);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_Timer_timeout(&mut self, owner: TRef<Node2D>) {
        self.set_invincible(owner, false);
    }
}