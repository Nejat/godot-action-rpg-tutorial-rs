#[macro_export]
macro_rules! array_item {
    ($ary:ident [ $index:literal ] : $type:ty) => {
        unsafe {
            $ary.get_ref($index)
                .try_to_object::<$type>()
                .expect(concat!(stringify!($type), " at ", stringify!($ary), "[", stringify!($index), "]"))
                .assume_safe()
        }
    };
    ($ary:ident [ $index:ident ] : $type:ty) => {
        unsafe {
            $ary.get_ref($index)
                .try_to_object::<$type>()
                .expect(concat!(stringify!($type), " at ", stringify!($ary), "[", stringify!($index), "]"))
                .assume_safe()
        }
    };
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

#[macro_export]
macro_rules! auto_load {
    (claim $node:literal : $type:ty) => {
        unsafe { autoload::<$type>($node).unwrap().claim() }
    };
    ($node:literal : $type:ty) => {
        unsafe { autoload::<$type>($node).unwrap() }
    };
    (claim $node:ident : $type:ty) => {
        unsafe { autoload::<$type>($node).unwrap().claim() }
    };
    ($node:ident : $type:ty) => {
        unsafe { autoload::<$type>($node).unwrap() }
    };
}

#[macro_export]
macro_rules! blend_position {
    ($param:literal) => {
        concat!("parameters/", $param, "/blend_position")
    };
}

#[macro_export]
macro_rules! call {
    ($src:expr; $method:literal) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[])
        }
    };
    ($src:expr; $method:literal : $type:ty) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[]).try_to_object::<$type>()
        }
    };
    ($src:expr; $method:ident) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[])
        }
    };
    ($src:expr; $method:ident : $type:ty) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[]).try_to_object::<$type>()
        }
    };
    ($src:expr; $method:literal ( $($arg:expr),* )) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[$($arg),*])
        }
    };
    ($src:expr; $method:literal ( $($arg:expr),* ) : $type:ty) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[$($arg),*]).try_to_object::<$type>()
        }
    };
    ($src:expr; $method:ident ( $($arg:expr),* )) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[$($arg),*])
        }
    };
    ($src:expr; $method:ident ( $($arg:expr),* ) : $type:ty) => {
        unsafe {
            $src.as_ref().unwrap().assume_safe().call($method, &[$($arg),*]).try_to_object::<$type>()
        }
    };
}

#[macro_export]
macro_rules! child_node {
    (claim $owner:ident [ $child:literal ] : $type:ty) => {
        unsafe {
            $owner.get_node_as::<$type>($child)
                .expect(concat!("\"", $child, "\" ", stringify!($type), " Child Node"))
        }.claim()
    };
    ($owner:ident [ $child:literal ] ) => {
        $owner.get_node($child)
            .expect(concat!("\"", $child, "\" Child Node"))
    };
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
    ($var:ident : $type:ty = $source:ident [ @ $param:literal ]) => {
        let $var = $source.get(concat!("parameters/", $param))
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"parameters/", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:expr ; @ $param:literal) => {
        let $var = $source.get(concat!("parameters/", $param))
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"parameters/", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:ident [ @ $param:ident ]) => {
        let $var = $source.get(concat!("parameters/", $param))
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"parameters/", stringify!($param), "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:expr ; @ $param:ident) => {
        let $var = $source.get(concat!("parameters/", $param))
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"parameters/", stringify!($param), "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:ident [ $param:literal ]) => {
        let $var = $source.get($param)
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:expr; $param:literal) => {
        let $var = $source.get($param)
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"", $param, "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:ident [ $param:ident ]) => {
        let $var = $source.get($param)
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"", stringify!($param), "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($var:ident : $type:ty = $source:expr; $param:ident) => {
        let $var = $source.get($param)
            .try_to_object::<$type>()
            .expect(concat!("\"", stringify!($var), ": ", stringify!($type), "\" getting parameter ", "\"", stringify!($param), "\""));
        let $var = unsafe { $var.assume_safe() };
    };
    ($source:ident [ $param:literal ]) => {{
        unsafe { $source.assume_safe() }.get($param)
    }};
    ($source:expr ; $param:literal) => {{
        unsafe { $source.assume_safe() }.get($param)
    }};
    ($source:ident [ $param:ident ]) => {{
        unsafe { $source.assume_safe() }.get($param)
    }};
    ($source:expr ; $param:ident) => {{
        unsafe { $source.assume_safe() }.get($param)
    }};
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
    (? $var:expr; $($param:literal = $value:expr),*) => {unsafe {
        let argument = $var.as_ref().unwrap().assume_safe();
        $(
            argument.set($param, $value);
        )*
    }};
    (? $var:expr; $($param:ident = $value:expr),*) => {unsafe {
        let argument = $var.as_ref().unwrap().assume_safe();
        $(
            argument.set($param, $value);
        )*
    }};
    (? $var:expr; @ $($param:literal = $value:expr),*) => {unsafe {
        let argument = $var.as_ref().unwrap().assume_safe();
        $(
            argument.set(concat!("parameters/", $param), $value);
        )*
    }};
    (? $var:expr; @ $($param:ident = $value:expr),*) => {unsafe {
        let argument = $var.as_ref().unwrap().assume_safe();
        $(
            argument.set(concat!("parameters/", $param), $value);
        )*
    }};
    ($var:expr; $($param:literal = $value:expr),*) => {unsafe {
        let argument = $var.assume_safe();
        $(
            argument.set($param, $value);
        )*
    }};
    ($var:expr; $($param:ident = $value:expr),*) => {unsafe {
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
    ($var:expr; @ $($param:ident = $value:expr),*) => {unsafe {
        let argument = $var.assume_safe();
        $(
            argument.set(concat!("parameters/", $param), $value);
        )*
    }};
}
