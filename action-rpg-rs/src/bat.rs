use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;
use crate::get_parameter;
use crate::has_effect::HasEffect;
use crate::load_resource;
use crate::set_parameter;
use crate::stats::PROPERTY_HEALTH;
use crate::sword::{PROPERTY_DAMAGE, PROPERTY_KNOCK_BACK_VECTOR};

pub(crate) const PROPERTY_ACCELERATION: &str = "acceleration";
pub(crate) const PROPERTY_FRICTION: &str = "friction";
pub(crate) const PROPERTY_KNOCK_BACK_FORCE: &str = "knock_back_force";
pub(crate) const PROPERTY_MAX_SPEED: &str = "max_speed";

const DEFAULT_ACCELERATION: f32 = 300.0;
const DEFAULT_FRICTION: f32 = 200.0;
const DEFAULT_KNOCK_BACK_FORCE: f32 = 120.0;
const DEFAULT_MAX_SPEED: f32 = 50.0;

#[allow(dead_code)]
enum BatState {
    CHASE,
    IDLE,
    WANDER,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register)]
pub struct Bat {
    #[property]
    acceleration: f32,
    effect: Option<Ref<PackedScene>>,
    #[property]
    friction: f32,
    knock_back: Vector2,
    #[property]
    knock_back_force: f32,
    #[property]
    max_speed: f32,
    state: BatState,
    stats: Option<Ref<Node>>,
    velocity: Vector2
}

impl HasEffect for Bat {
    fn effect_scene(&self) -> &Option<Ref<PackedScene>> {
        &self.effect
    }
}

impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Bat {
            acceleration: DEFAULT_ACCELERATION,
            effect: None,
            friction: DEFAULT_FRICTION,
            knock_back: Vector2::zero(),
            knock_back_force: DEFAULT_KNOCK_BACK_FORCE,
            max_speed: DEFAULT_MAX_SPEED,
            state: BatState::CHASE,
            stats: None,
            velocity: Vector2::zero()
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<f32>(PROPERTY_ACCELERATION)
            .with_getter(|s: &Self, _| s.acceleration)
            .with_setter(|s: &mut Self, _, value: f32| s.acceleration = value)
            .with_default(DEFAULT_ACCELERATION)
            .done();

        builder
            .add_property::<f32>(PROPERTY_FRICTION)
            .with_getter(|s: &Self, _| s.friction)
            .with_setter(|s: &mut Self, _, value: f32| s.friction = value)
            .with_default(DEFAULT_FRICTION)
            .done();

        builder
            .add_property::<f32>(PROPERTY_KNOCK_BACK_FORCE)
            .with_getter(|s: &Self, _| s.knock_back_force)
            .with_setter(|s: &mut Self, _, value: f32| s.knock_back_force = value)
            .with_default(DEFAULT_KNOCK_BACK_FORCE)
            .done();

        builder
            .add_property::<f32>(PROPERTY_MAX_SPEED)
            .with_getter(|s: &Self, _| s.max_speed)
            .with_setter(|s: &mut Self, _, value: f32| s.max_speed = value)
            .with_default(DEFAULT_MAX_SPEED)
            .done();
    }
}

#[methods]
impl Bat {
    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        load_resource! { scene: PackedScene = "Effects/EnemyDeathEffect.tscn" {
            self.effect = Some(scene.claim())
        } }

        child_node! { stats = owner["Stats"] }

        self.stats = Some(stats)
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        self.knock_back = owner.move_and_slide(
            self.knock_back.move_towards(Vector2::zero(), self.friction * delta),
            Vector2::zero(), false, 4, FRAC_PI_4, true
        );

        match self.state {
            BatState::CHASE => {}
            BatState::IDLE =>
                self.velocity = self.velocity.move_towards(Vector2::zero(), self.friction * delta),
            BatState::WANDER => {}
        }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, _owner: &KinematicBody2D, area: Ref<Area2D>) {
        let damage = get_parameter!(area[PROPERTY_DAMAGE]).to_i64();
        let health = get_parameter!(self.stats.unwrap(); PROPERTY_HEALTH).to_i64() - damage;

        set_parameter!(self.stats.unwrap(); PROPERTY_HEALTH = health);
        self.knock_back = get_parameter!(area[PROPERTY_KNOCK_BACK_VECTOR]).to_vector2() * self.knock_back_force;
    }

    // when connecting signal in the godot editor, click the "advanced" switch
    // and select the "deferred" option, otherwise an exception occurs
    // todo: figure out why this is necessary
    #[export]
    #[allow(non_snake_case)]
    fn _on_Stats_no_health(&self, owner: &KinematicBody2D) {
        self.play_effect_parent(owner);
        owner.queue_free();
    }
}
