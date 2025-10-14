use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
pub struct Effect;

impl Effect {
    fn new(_owner: TRef<AnimatedSprite>) -> Self {
        Effect
    }
}

#[methods]
impl Effect {
    #[method]
    fn _ready(&mut self, #[base] owner: TRef<AnimatedSprite>) {
        owner
            .connect(
                "animation_finished",
                owner,
                "_on_animation_finished",
                VariantArray::new_shared(),
                1,
            )
            .expect("_on_animation_finished to connect to effect instance");

        owner.set_frame(0);
        owner.play("Animate", false);
    }

    #[method]
    #[allow(non_snake_case)]
    fn _on_animation_finished(&mut self, #[base] owner: TRef<AnimatedSprite>) {
        owner.queue_free();
    }
}
