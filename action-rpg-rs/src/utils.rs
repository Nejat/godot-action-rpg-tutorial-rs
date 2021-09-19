#[macro_export]
macro_rules! child_node {
    ($var :ident: $type:ty = $owner:ident [ $child:literal ] ) => {
        let $var = unsafe {
            $owner.get_node_as::<$type>($child)
                .expect(concat!("\"", $child, "\" ", stringify!($type), " Child Node"))
        };
    };
}

#[macro_export]
macro_rules! get_parameter {
    ($var:ident : $type:ty = $source:ident [ $param:literal ]) => {
        let $var = $source.get(concat!("parameters/", $param))
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"parameters/", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    }
}
