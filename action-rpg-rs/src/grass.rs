use gdnative::api::*;
use gdnative::prelude::*;

use crate::assume_safe;
use crate::load_resource;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass {
    effect: Option<Ref<PackedScene>>
}

impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Grass { effect: None }
    }
}

#[methods]
impl Grass {
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        load_resource! { scene: PackedScene = "Effects/GrassEffect.tscn" {
            self.effect = Some(scene.claim())
        } }
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, owner: &Node2D, _area: Ref<Area2D>) {
        let scene = assume_safe!(self.effect);

        assume_safe! {
            let instance: Node2D = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED),
            let root: SceneTree = Node::get_tree(&owner),
            let current: Node = root.current_scene() => {
                current.add_child(instance, false);
                instance.set_global_position(owner.global_position());
            }
        }

        owner.queue_free();
    }
}