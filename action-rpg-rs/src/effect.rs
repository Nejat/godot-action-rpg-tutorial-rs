use godot::classes::{AnimatedSprite2D, IAnimatedSprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AnimatedSprite2D)]
pub struct Effect {
    base: Base<AnimatedSprite2D>,
}

#[godot_api]
impl IAnimatedSprite2D for Effect {
    fn init(base: Base<AnimatedSprite2D>) -> Self {
        Effect { base }
    }

    fn ready(&mut self) {
        let callable = self.base().callable("_on_animation_finished");

        self.base_mut().connect("animation_finished", &callable);
        self.base_mut().set_frame(0);
        self.base_mut().play_ex().name("Animate").done();
    }
}

#[godot_api]
impl Effect {
    #[func]
    fn _on_animation_finished(&mut self) {
        self.base_mut().queue_free();
    }
}
