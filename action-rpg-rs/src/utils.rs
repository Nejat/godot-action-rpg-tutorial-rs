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

#[macro_export]
macro_rules! load_resource {
    ($res:ident : $type:ty = $src:literal $code:block) => {
        if let Some($res) = ResourceLoader::godot_singleton()
            .load(concat!("res://", $src), stringify!($type), false)
            .and_then(|res| res.cast::<$type>()) {
            let $res = unsafe { $res.assume_safe() };
            $code
        }
    };
    ($res:ident : $type:ty = $src:expr => $code:block) => {
        if let Some($res) = ResourceLoader::godot_singleton()
            .load($src, stringify!($type), false)
            .and_then(|res| res.cast::<$type>())  {
            let $res = unsafe { $res.assume_safe() };
            $code
        }
    };
    ($src:literal: $type:ty) => {
        unsafe {
            ResourceLoader::godot_singleton()
                .load(concat!("res://", $src), stringify!($type), false)
                .and_then(|res| res.cast::<$type>())
                .assume_safe()
        }
    };
    ($src:expr; $type:ty) => {
        unsafe {
            ResourceLoader::godot_singleton()
                .load($src, stringify!($type), false)
                .and_then(|res| res.cast::<$type>())
                .assume_safe()
        }
    };
}

#[macro_export]
macro_rules! assume_safe_if {
    (let $var:ident : $type:ty = $src: expr => $code: block) => {
        if let Some($var) = $src.and_then(|v| { let v = unsafe { v.assume_safe() }; v.cast::<$type>() }) {
            $code
        }
    };
    (let $var:ident = $src: expr => $code: block) => {
        if let Some($var) = $src.and_then(|v| { let v = unsafe { v.assume_safe() }; Some(v) }) {
            $code
        }
    };
}