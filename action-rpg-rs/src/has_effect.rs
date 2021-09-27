use gdnative::api::*;
use gdnative::prelude::*;

use crate::assume_safe;

pub(crate) trait HasEffect {
    fn effect_scene(&self) -> &Option<Ref<PackedScene>>;

    #[inline]
    fn play_effect_parent(&self, owner: &Node2D) {
        let scene = self.effect_scene();
        let scene = assume_safe!(scene);

        assume_safe! {
            let instance: Node2D = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED),
            let parent: Node = Node::get_parent(owner) => {
                parent.add_child(instance, false);
                instance.set_global_position(owner.global_position());
            }
        }
    }

    #[inline]
    fn play_effect_root(&self, owner: &Node2D) {
        let scene = assume_safe!(self.effect_scene());

        assume_safe! {
            let instance: Node2D = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED),
            let root: SceneTree = Node::get_tree(owner),
            let scene: Node = root.current_scene() => {
                scene.add_child(instance, false);
                instance.set_global_position(owner.global_position());
            }
        }
    }
}