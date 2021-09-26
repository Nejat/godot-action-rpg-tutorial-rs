use gdnative::api::*;
use gdnative::nativescript::property::Usage;
use gdnative::prelude::*;

pub(crate) const METHOD_CAN_SEE_PLAYER: &str = "can_see_player";
pub(crate) const METHOD_GET_PLAYER: &str = "get_player";

pub(crate) const PROPERTY_PLAYER: &str = "player";

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register)]
pub struct PlayerDetectionZone {
    #[property(no_editor)]
    player: Option<Ref<Node>>,
}

impl PlayerDetectionZone {
    fn new(_owner: &Area2D) -> Self {
        PlayerDetectionZone {
            player: None
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<Option<Ref<Node>>>(PROPERTY_PLAYER)
            .with_getter(|s: &Self, _| s.player)
            .with_setter(|s: &mut Self, _, value: Option<Ref<Node>>| s.player = value)
            .with_usage(Usage::NOEDITOR)
            .done();
    }
}

#[methods]
impl PlayerDetectionZone {
    #[export]
    pub(crate) fn can_see_player(&self, _owner: &Area2D) -> bool { self.player.is_some() }

    #[export]
    pub(crate) fn get_player(&self, _owner: &Area2D) -> Variant { self.player.to_variant() }

    #[export]
    #[allow(non_snake_case)]
    fn _on_PlayerDetectionZone_body_entered(&mut self, _owner: &Area2D, body: Ref<Node>) {
        self.player = Some(body);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_PlayerDetectionZone_body_exited(&mut self, _owner: &Area2D, _body: Ref<Node>) {
        self.player = None
    }
}