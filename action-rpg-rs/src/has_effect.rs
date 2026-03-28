use godot::classes::PackedScene;
use godot::prelude::*;

pub(crate) trait HasEffect {
    fn effect_scene(&self) -> &Option<Gd<PackedScene>>;

    #[inline]
    fn play_effect_parent(&self, owner: &Gd<Node2D>) {
        if let Some(scene) = self.effect_scene()
            && let Some(instance) = scene.instantiate()
        {
            let global_pos = owner.get_global_position();

            if let Some(mut parent) = owner.get_parent() {
                parent.add_child(&instance);

                let mut instance_2d: Gd<Node2D> = instance.cast();

                instance_2d.set_global_position(global_pos);
            }
        }
    }

    #[inline]
    fn play_effect_root(&self, owner: &Gd<Node2D>) {
        if let Some(scene) = self.effect_scene()
            && let Some(instance) = scene.instantiate()
        {
            let global_pos = owner.get_global_position();
            let tree = owner.get_tree();

            if let Some(mut current_scene) = tree.get_current_scene() {
                current_scene.add_child(&instance);

                let mut instance_2d: Gd<Node2D> = instance.cast();

                instance_2d.set_global_position(global_pos);
            }
        }
    }
}
