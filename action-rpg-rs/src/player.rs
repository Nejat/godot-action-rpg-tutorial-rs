use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::{
    assume_safe, auto_load, blend_position, call,
    child_node, get_parameter, load_resource, set_parameter,
};
use crate::hurt_box::{METHOD_PLAY_HIT_EFFECT, METHOD_START_INVINCIBILITY};
use crate::stats::{PROPERTY_HEALTH, SIGNAL_NO_HEALTH};
use crate::sword::PROPERTY_KNOCK_BACK_VECTOR;

type AnimationPlayback = AnimationNodeStateMachinePlayback;

pub(crate) const PROPERTY_ACCELERATION: &str = "acceleration";
pub(crate) const PROPERTY_FRICTION: &str = "friction";
pub(crate) const PROPERTY_MAX_SPEED: &str = "max_speed";
pub(crate) const PROPERTY_ROLL_SPEED: &str = "roll_speed";

const DEFAULT_ACCELERATION: f32 = 500.0;
const DEFAULT_FRICTION: f32 = 500.0;
const DEFAULT_MAX_SPEED: f32 = 80.0;
const DEFAULT_ROLL_SPEED: f32 = 120.0;

const INPUT_ATTACK: &str = "ui_attack";
const INPUT_DOWN: &str = "ui_down";
const INPUT_LEFT: &str = "ui_left";
const INPUT_RIGHT: &str = "ui_right";
const INPUT_ROLL: &str = "ui_roll";
const INPUT_UP: &str = "ui_up";

const TRAVEL_ATTACK: &str = "Attack";
const TRAVEL_IDLE: &str = "Idle";
const TRAVEL_ROLL: &str = "Roll";
const TRAVEL_RUN: &str = "Run";

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum PlayerState {
    Attack,
    Move,
    Roll,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register)]
pub struct Player {
    #[property]
    acceleration: f32,
    // todo: when using Ref<T>, the placeholder values in "fn new" cause the following warning in godot
    // todo: "WARNING: cleanup: ObjectDB instances leaked at exit"
    animation_state: Option<Ref<AnimationPlayback>>,
    animation_tree: Option<Ref<AnimationTree>>,
    #[property]
    friction: f32,
    hurt_box: Option<Ref<Node2D>>,
    hurt_sound: Option<Ref<PackedScene>>,
    #[property]
    max_speed: f32,
    player_stats: Option<Ref<Node>>,
    #[property]
    roll_speed: f32,
    roll_vector: Vector2,
    state: PlayerState,
    sword: Option<Ref<Area2D>>,
    velocity: Vector2,
}

impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            acceleration: DEFAULT_ACCELERATION,
            animation_state: None,
            animation_tree: None,
            friction: DEFAULT_FRICTION,
            hurt_box: None,
            hurt_sound: None,
            max_speed: DEFAULT_MAX_SPEED,
            roll_speed: DEFAULT_ROLL_SPEED,
            roll_vector: Vector2::new(0.0, 1.0), // DOWN
            player_stats: None,
            state: PlayerState::Move,
            sword: None,
            velocity: Vector2::zero(),
        }
    }

    //noinspection DuplicatedCode
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
            .add_property::<f32>(PROPERTY_MAX_SPEED)
            .with_getter(|s: &Self, _| s.max_speed)
            .with_setter(|s: &mut Self, _, value: f32| s.max_speed = value)
            .with_default(DEFAULT_MAX_SPEED)
            .done();

        builder
            .add_property::<f32>(PROPERTY_ROLL_SPEED)
            .with_getter(|s: &Self, _| s.roll_speed)
            .with_setter(|s: &mut Self, _, value: f32| s.roll_speed = value)
            .with_default(DEFAULT_ROLL_SPEED)
            .done();
    }
}

// the additional values passed to godot functions, that are not mentioned in
// the video, are listed in the api documentation and are defaults in gdscript

#[methods]
impl Player {
    #[export]
    fn _ready(&mut self, owner: TRef<KinematicBody2D>) {
        let owner_ref = owner.as_ref();

        child_node! { animation_tree: AnimationTree = owner_ref["AnimationTree"] }
        get_parameter! { animation_state: AnimationPlayback = animation_tree[@"playback"] }

        animation_tree.set_active(true);

        self.animation_tree = Some(animation_tree.claim());
        self.animation_state = Some(animation_state.claim());
        self.hurt_box = Some(child_node!(claim owner_ref["HurtBox"]: Node2D));
        self.sword = Some(child_node!(claim owner_ref["HitboxPivot/SwordHitbox"]: Area2D));

        load_resource! { scene: PackedScene = "Player/PlayerHurtSound.tscn" {
            self.hurt_sound = Some(scene.claim())
        } }

        let player_stats = auto_load!("PlayerStats": Node);

        player_stats
            .connect(SIGNAL_NO_HEALTH, owner, "_on_Stats_no_health", VariantArray::new_shared(), 1)
            .expect("_on_Stats_no_health to connect to player stats");

        self.player_stats = Some(player_stats.claim());
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        match self.state {
            PlayerState::Move =>
                self.move_state(owner, delta),
            PlayerState::Attack =>
                self.attack_state(owner),
            PlayerState::Roll =>
                self.roll_state(owner)
        }
    }

    #[export]
    fn attack_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.state = PlayerState::Move
    }

    #[export]
    fn roll_animation_finished(&mut self, _owner: &KinematicBody2D) {
        self.velocity = self.velocity * 0.8; // ease sliding past roll animation
        self.state = PlayerState::Move
    }

    #[inline]
    fn attack_state(&mut self, _owner: &KinematicBody2D) {
        self.velocity = Vector2::zero();

        assume_safe!(self.animation_state).travel(TRAVEL_ATTACK);
    }

    #[inline]
    fn move_state(&mut self, owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        input_vector.x = (input.get_action_strength(INPUT_RIGHT) -
            input.get_action_strength(INPUT_LEFT)) as f32;

        input_vector.y = (input.get_action_strength(INPUT_DOWN) -
            input.get_action_strength(INPUT_UP)) as f32;

        if input_vector != Vector2::zero() {
            // in the video, the function "normalized" is used, which handles zero condition.
            // godot-rust does not have that function, instead there is a try_normalize.
            // since we only use the input_vector when it's none zero, I opted to use the
            // "normalize" function after the check for zero.
            input_vector = input_vector.normalize();

            self.roll_vector = input_vector;

            set_parameter! { ?self.sword; PROPERTY_KNOCK_BACK_VECTOR = input_vector }

            let animation_tree = assume_safe!(self.animation_tree);

            animation_tree.set(blend_position!("Idle"), input_vector);
            animation_tree.set(blend_position!("Run"), input_vector);
            animation_tree.set(blend_position!("Attack"), input_vector);
            animation_tree.set(blend_position!("Roll"), input_vector);

            assume_safe!(self.animation_state).travel(TRAVEL_RUN);

            self.velocity = self.velocity.move_towards(input_vector * self.max_speed, self.acceleration * delta);
        } else {
            assume_safe!(self.animation_state).travel(TRAVEL_IDLE);

            self.velocity = self.velocity.move_towards(Vector2::zero(), self.friction * delta);
        }

        self.move_player(owner);

        if input.is_action_just_pressed(INPUT_ROLL) {
            self.state = PlayerState::Roll
        }

        if input.is_action_just_pressed(INPUT_ATTACK) {
            self.state = PlayerState::Attack
        }
    }

    #[inline]
    fn roll_state(&mut self, owner: &KinematicBody2D) {
        self.velocity = self.roll_vector * self.roll_speed;

        assume_safe!(self.animation_state).travel(TRAVEL_ROLL);

        self.move_player(owner);
    }

    #[inline]
    fn move_player(&mut self, owner: &KinematicBody2D) {
        // FRAC_PI_4 was suggested by c-lion ide as an approximate constant of the
        // documented default value of 0.785398 for "floor_max_angle"

        self.velocity = owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, owner: &KinematicBody2D, _area: Ref<Area2D>) {
        let health = get_parameter!(self.player_stats.unwrap(); PROPERTY_HEALTH).to_i64() - 1;

        set_parameter!(self.player_stats.unwrap(); PROPERTY_HEALTH = health);

        call!(self.hurt_box; METHOD_START_INVINCIBILITY(0.5.to_variant()));
        call!(self.hurt_box; METHOD_PLAY_HIT_EFFECT);

        let scene = assume_safe!(self.hurt_sound);

        assume_safe! {
            let instance: Node = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED),
            let root: SceneTree = Node::get_tree(owner),
            let scene: Node = root.current_scene() => {
                scene.add_child(instance, false);
            }
        }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_Stats_no_health(&self, owner: &KinematicBody2D) {
        owner.queue_free();
    }
}
