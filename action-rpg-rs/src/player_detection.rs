use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PlayerDetectionZone {
    base: Base<Area2D>,
    player: Option<Gd<Node2D>>,
}

#[godot_api]
impl IArea2D for PlayerDetectionZone {
    fn init(base: Base<Area2D>) -> Self {
        PlayerDetectionZone { base, player: None }
    }
}

#[godot_api]
impl PlayerDetectionZone {
    #[func]
    pub fn can_see_player(&self) -> bool {
        self.player.is_some()
    }

    #[func]
    pub fn get_player(&self) -> Variant {
        match &self.player {
            Some(p) => p.to_variant(),
            None => Variant::nil(),
        }
    }

    #[func]
    fn _on_player_detection_zone_body_entered(&mut self, body: Gd<Node2D>) {
        self.player = Some(body);
    }

    #[func]
    fn _on_player_detection_zone_body_exited(&mut self, _body: Gd<Node2D>) {
        self.player = None;
    }
}
