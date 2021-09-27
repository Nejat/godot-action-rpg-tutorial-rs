use std::f64::consts::FRAC_PI_4;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::child_node;
use crate::get_parameter;
use crate::assume_safe;
use crate::set_parameter;

type AnimationPlayback = AnimationNodeStateMachinePlayback;

pub(crate) const PROPERTY_ACCELERATION: &str = "acceleration";
pub(crate) const PROPERTY_FRICTION: &str = "friction";
pub(crate) const PROPERTY_MAX_SPEED: &str = "max_speed";
pub(crate) const PROPERTY_ROLL_SPEED: &str = "roll_speed";

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
    #[property]
    max_speed: f32,
    #[property]
    roll_speed: f32,
    roll_vector: Vector2,
    state: PlayerState,
    sword: Option<Ref<Area2D>>,
    velocity: Vector2,
}

const DEFAULT_ACCELERATION: f32 = 500.0;
const DEFAULT_FRICTION: f32 = 500.0;
const DEFAULT_MAX_SPEED: f32 = 80.0;
const DEFAULT_ROLL_SPEED: f32 = 120.0;

impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            acceleration: DEFAULT_ACCELERATION,
            animation_state: None,
            animation_tree: None,
            friction: DEFAULT_FRICTION,
            max_speed: DEFAULT_MAX_SPEED,
            roll_speed: DEFAULT_ROLL_SPEED,
            roll_vector: Vector2::new(0.0, 1.0), // DOWN
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
    fn _ready(&mut self, owner: &KinematicBody2D) {
        child_node! { animation_tree: AnimationTree = owner["AnimationTree"] }
        get_parameter! { animation_state: AnimationPlayback = animation_tree[@"playback"] };

        animation_tree.set_active(true);

        self.animation_tree = Some(animation_tree.claim());
        self.animation_state = Some(animation_state.claim());
        self.sword = Some(child_node!(claim owner["HitboxPivot/SwordHitbox"]: Area2D));
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

        assume_safe!(self.animation_state).travel("Attack");
    }

    #[inline]
    fn move_state(&mut self, owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        input_vector.x = (input.get_action_strength("ui_right") -
            input.get_action_strength("ui_left")) as f32;

        input_vector.y = (input.get_action_strength("ui_down") -
            input.get_action_strength("ui_up")) as f32;
        if input_vector != Vector2::zero() {
            // in the video, the function "normalized" is used, which handles zero condition.
            // godot-rust does not have that function, instead there is a try_normalize.
            // since we only use the input_vector when it's none zero, I opted to use the
            // "normalize" function after the check for zero.
            input_vector = input_vector.normalize();

            self.roll_vector = input_vector;

            set_parameter!{ ?self.sword; "knock_back_vector" = input_vector }

            assume_safe!(self.animation_tree).set("parameters/Idle/blend_position", input_vector);
            assume_safe!(self.animation_tree).set("parameters/Run/blend_position", input_vector);
            assume_safe!(self.animation_tree).set("parameters/Attack/blend_position", input_vector);
            assume_safe!(self.animation_tree).set("parameters/Roll/blend_position", input_vector);

            assume_safe!(self.animation_state).travel("Run");

            self.velocity = self.velocity.move_towards(input_vector * self.max_speed, self.acceleration * delta);
        } else {
            assume_safe!(self.animation_state).travel("Idle");

            self.velocity = self.velocity.move_towards(Vector2::zero(), self.friction * delta);
        }

        self.move_player(owner);

        if input.is_action_just_pressed("ui_roll") {
            self.state = PlayerState::Roll
        }

        if input.is_action_just_pressed("ui_attack") {
            self.state = PlayerState::Attack
        }
    }

    #[inline]
    fn roll_state(&mut self, owner: &KinematicBody2D) {
        self.velocity = self.roll_vector * self.roll_speed;

        assume_safe!(self.animation_state).travel("Roll");

        self.move_player(owner);
    }

    #[inline]
    fn move_player(&mut self, owner: &KinematicBody2D) {
        // FRAC_PI_4 was suggested by c-lion ide as an approximate constant of the
        // documented default value of 0.785398 for "floor_max_angle"

        self.velocity = owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, FRAC_PI_4, true);
    }
}
