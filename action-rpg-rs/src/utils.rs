#[macro_export]
macro_rules! child_node {
    ($var:ident: $type:ty = $owner:ident [ $child:literal ] ) => {
        let $var = unsafe {
            $owner.get_node_as::<$type>($child)
                .expect(concat!("\"", $child, "\" ", stringify!($type), " Child Node"))
        };
    };
    ($var:ident = $owner:ident [ $child:literal ] ) => {
        let $var = $owner.get_node($child)
            .expect(concat!("\"", $child, "\" Child Node"));
    };
}

#[macro_export]
macro_rules! get_parameter {
    ($source:ident [ $param:literal ] ) => {{
        unsafe { $source.assume_safe() }.get($param)
    }};
    ($var:ident : $type:ty = $source:ident [ @ $param:literal ]) => {
        let $var = $source.get(concat!("parameters/", $param))
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"parameters/", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:ident [ $param:literal ]) => {
        let $var = $source.get($param)
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    };
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
macro_rules! set_parameter {
    (?$var:expr; $($param:literal = $value:expr),*) => {unsafe {
        let argument = $var.as_ref().unwrap().assume_safe();
        $(
            argument.set($param, $value);
        )*
    }};
    ($var:expr; $($param:literal = $value:expr),*) => {unsafe {
        let argument = $var.assume_safe();
        $(
            argument.set($param, $value);
        )*
    }};
    ($var:expr; @ $($param:literal = $value:expr),*) => {unsafe {
        let argument = $var.assume_safe();
        $(
            argument.set(concat!("parameters/", $param), $value);
        )*
    }};
}

#[macro_export]
macro_rules! assume_safe {
    ($var:expr) => {
        unsafe { $var.as_ref().unwrap().assume_safe() }
    };
    ($(let $var:ident : $type:ty = $src:expr),* => $code:block) => {
        $(
            let $var: TRef<$type> = $src
                .and_then(|v| { unsafe { v.assume_safe().cast::<$type>() } })
                .expect(concat!(stringify!($var), ":", stringify!($type), " = ", stringify!($src)));
        )*
        $code
    };
}
