#[macro_export]
macro_rules! child_node {
    ($owner:ident, $child:literal => $dest:ident: $type:ty) => {
        let $dest = unsafe {
            $owner.get_node($child)
                .expect(&format!("{} child node is required for player", $child))
                .assume_safe()
                .cast::<$type>()
                .expect(&format!("{} node", $child))
        };
    }
}

#[macro_export]
macro_rules! get_parameter {
    ($source:ident, $param:literal => $dest:ident: $type:ty) => {
        let $dest = $source.get($param)
            .try_to_object::<$type>()
            .expect($param);
        let $dest = unsafe { $dest.assume_safe() };
    }
}
