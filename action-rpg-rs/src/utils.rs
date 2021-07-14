use gdnative::prelude::*;

//noinspection RsSelfConvention
pub fn get_node<'a, T>(owner: &KinematicBody2D, name: &str) -> TRef<'a, T>
    where T: GodotObject + SubClass<Node>
{
    unsafe {
        owner.get_node(name)
            .expect(&format!("{} child node is required for player", name))
            .assume_safe()
            .cast::<T>()
            .expect(&format!("{} node", name))
    }
}
