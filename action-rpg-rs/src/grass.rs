use gdnative::api::*;
use gdnative::prelude::*;

use crate::assume_safe_if;
use crate::load_resource;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass;

impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Grass
    }
}

#[methods]
impl Grass {
    #[export]
    #[allow(non_snake_case)]
    fn _on_HurtBox_area_entered(&mut self, owner: &Node2D, _area: Ref<Area2D>) {
        load_resource! { scene: PackedScene = "Effects/GrassEffect.tscn" {
            assume_safe_if! { let instance: Node2D = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED) => {
                assume_safe_if! { let root = Node::get_tree(&owner) => {
                    assume_safe_if! { let current = root.current_scene()  => {
                        current.add_child(instance, false);
                        instance.set_global_position(owner.global_position());
                    } }
                } }
            } }
        } }

        owner.queue_free();
    }
}