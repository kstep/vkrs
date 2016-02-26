#![macro_use]

macro_rules! qs {
    ($($name:expr => $value:expr),+ $(,)*) => {
        ::url::form_urlencoded::serialize([
            $(($name, $value)),*
        ].into_iter().filter(|&&(_, v)| !v.is_empty()))
    }
}

macro_rules! expand_value_expr {
    ($this:ident; $param_name:ident; =) => {
        expand_value_expr!($this; $param_name; |value| *value)
    };
    ($this:ident; $param_name:ident;) => {
        expand_value_expr!($this; $param_name; ToString)
    };
    ($this:ident; $param_name:ident; ToString) => {
        expand_value_expr!($this; $param_name; |value|
                           &*value.to_string())
    };
    ($this:ident; $param_name:ident; Borrow) => {
        expand_value_expr!($this; $param_name; |value|
                           value.borrow())
    };
    ($this:ident; $param_name:ident; AsRef) => {
        expand_value_expr!($this; $param_name; |value|
                           value.as_ref())
    };
    ($this:ident; $param_name:ident; bool) => {
        expand_value_expr!($this; $param_name; |value|
                           if *value {"1"} else {"0"})
    };
    ($this:ident; $param_name:ident; AsRef<Vec>) => {
        expand_value_expr!($this; $param_name; |value|
                           &*value.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(","))
    };
    ($this:ident; $param_name:ident; AsRef<Option>) => {
        expand_value_expr!($this; $param_name; |value|
                           &*value.as_ref().map(AsRef::as_ref).unwrap_or(""))
    };
    ($this:ident; $param_name:ident; Vec) => {
        expand_value_expr!($this; $param_name; |value|
                           &*value.iter().map(ToString::to_string).collect::<Vec<_>>().join(","))
    };
    ($this:ident; $param_name:ident; Option) => {
        expand_value_expr!($this; $param_name; |value|
                           value.as_ref().map(ToString::to_string).as_ref()
                           .map(Borrow::borrow).unwrap_or(""))
    };
    ($this:ident; $param_name:ident; |$value_name:ident| $value_expr:expr) => {
        { let $value_name = &$this.$param_name; $value_expr }
    };
}

macro_rules! expand_init_expr {
    (()) => { Default::default() };
    ({}) => { Default::default() };
    (($value:expr)) => { $value };
    ({$value:expr}) => { $value };
}

macro_rules! request_builder_impl {
    (
        $struct_name:ident
        {
            $($param_name:ident: $param_type:ty = $param_value:tt),*
            $(,)*
        }
    ) => {
        #[allow(non_camel_case_types)]
        pub fn new() -> Self {
            $struct_name {
                $($param_name: expand_init_expr!($param_value),)*
            }
        }
        $(request_builder_setter_impl!($param_name: $param_type $param_value);)*
    }
}

macro_rules! request_builder_setter_impl {
    ($param_name:ident: $param_type:ty {$param_value:expr}) => { request_builder_setter_impl!($param_name: $param_type {}); };
    ($param_name:ident: $param_type:ty ($param_value:expr)) => { request_builder_setter_impl!($param_name: $param_type ()); };
    (
        $param_name:ident: $param_type:ty {}
    ) => {
        pub fn $param_name<T: Into<$param_type>>(&mut self, value: T) -> &mut Self {
            self.$param_name = value.into();
            self
        }
    };
    (
        $param_name:ident: $param_type:ty ()
    ) => {
        pub fn $param_name(&mut self, value: $param_type) -> &mut Self {
            self.$param_name = value;
            self
        }
    };
}

macro_rules! request_trait_impl {
    (
        [$method_name:expr]($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty[$($permission:ident),*]
        {
            $($param_name:ident as $param_alias:expr => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        type Response = $response_type;
        fn method_name() -> &'static str { $method_name }
        fn permissions() -> ::auth::Permissions { ::auth::Permissions::new($(::auth::Permission::$permission as i32 |)* 0) }
        fn to_query_string(&self) -> String {
            qs![
                $($param_alias => expand_value_expr!(self; $param_name; $($value)*),)*
                $(stringify!($const_param_name) => concat!($const_param_value),)*
            ]
        }
    };

    (
        [$method_name:expr]($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty[$($permission:ident),*]
        {
            $($param_name:ident => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        request_trait_impl! {
            [$method_name]($($const_param_name => $const_param_value),*) ->
            $response_type [$($permission),*]
            {
                $($param_name as stringify!($param_name) => {$($value)*}),*
            }
        }
    };
}

macro_rules! request {
    // Empty struct without permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty;
    ) => {
        request! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type [];
        }
    };

    // Empty struct with permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*];
    ) => {
        #[derive(Debug, PartialEq, Clone, Copy, Eq)]
        $(#[$attr])*
        pub struct $struct_name;

        impl ::api::Request for $struct_name {
            request_trait_impl! {
                [$method_name]
                ($($const_param_name => $const_param_value),*)
                -> $response_type [$($permission),*] {}
            }
        }
    };

    // Struct without both aliased fields and permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            $($param_name:ident: $param_type:ty = $param_value:tt => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        request! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type []
            {
                $($param_name as (stringify!($param_name)): $param_type = $param_value => {$($value)*}),*
            }
        }
    };

    // Struct without aliased fields, with permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*]
        {
            $($param_name:ident: $param_type:ty = $param_value:tt => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        request! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type [$($permission),*]
            {
                $($param_name as (stringify!($param_name)): $param_type = $param_value => {$($value)*}),*
            }
        }
    };

    // Struct with aliased fields, without permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            $($param_name:ident as ($param_alias:expr): $param_type:ty = $param_value:tt => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        request! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type []
            {
                $($param_name as ($param_alias): $param_type = $param_value => {$($value)*}),*
            }
        }
    };

    // Struct with aliased fields and permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*]
        {
            $($param_name:ident as ($param_alias:expr): $param_type:ty = $param_value:tt => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        $(#[$attr])*
        pub struct $struct_name {
            $($param_name: $param_type,)*
        }

        impl ::api::Request for $struct_name {
            request_trait_impl! {
                [$method_name]
                ($($const_param_name => $const_param_value),*)
                -> $response_type [$($permission),*]
                {
                    $($param_name as $param_alias => {$($value)*},)*
                }
            }
        }

        impl $struct_name {
            request_builder_impl! {
                $struct_name
                {
                    $($param_name: $param_type = $param_value),*
                }
            }
        }
    };
}

macro_rules! request_ref {
    // Struct without permissions, aliases and sized fields
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            $($param_name_lt:ident: $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)*
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type []
            {
                $($param_name_lt as (stringify!($param_name_lt)): $param_type_lt = $param_value_lt => {$($value_lt)*}),*
            }
        }
    };

    // Struct with sized fields, without permissions and aliases
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            sized { $($param_name:ident: $param_type:ty = $param_value:tt => {$($value:tt)*}),* $(,)* }
            unsized { $($param_name_lt:ident: $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)* }
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type []
            {
                sized {
                    $($param_name as (stringify!($param_name)): $param_type = $param_value => {$($value)*}),*
                }
                unsized {
                    $($param_name_lt as (stringify!($param_name_lt)): $param_type_lt = $param_value_lt => {$($value_lt)*}),*
                }
            }
        }
    };

    // Struct with aliases, without sized fields and permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            $($param_name_lt:ident as ($param_alias_lt:expr): $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)*
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type []
            {
                sized {
                }
                unsized {
                    $($param_name_lt as ($param_alias_lt): $param_type_lt = $param_value_lt => {$($value_lt)*}),*
                }
            }
        }
    };

    // Struct with permissions, without sized fields and aliases
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*]
        {
            $($param_name_lt:ident: $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)*
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type [$($permission),*]
            {
                sized {
                }
                unsized {
                    $($param_name_lt: $param_type_lt = $param_value_lt => {$($value_lt)*}),*
                }
            }
        }
    };

    // Struct with permissions and aliases, without sized fields
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*]
        {
            $($param_name_lt:ident as ($param_alias_lt:expr): $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)*
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type [$($permission:ident),*]
            {
                sized {
                }
                unsized {
                    $($param_name_lt as ($param_alias_lt): $param_type_lt = $param_value_lt => {$($value_lt)*}),*
                }
            }
        }
    };

    // Struct with sized fields and permissions, without aliases
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*]
        {
            sized {$($param_name:ident: $param_type:ty = $param_value:tt => {$($value:tt)*}),* $(,)*}
            unsized {$($param_name_lt:ident: $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)*}
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type [$($permission),*]
            {
                sized { $($param_name as (stringify!($param_name)): $param_type = $param_value => {$($value)*}),* }
                unsized { $($param_name_lt as (stringify!($param_name_lt)): $param_type_lt = $param_value_lt => {$($value_lt)*}),* }
            }
        }
    };

    // Struct with sized fields and aliases, without permissions
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            sized {
                $($param_name:ident as ($param_alias:expr): $param_type:ty = $param_value:tt => {$($value:tt)*}),*
                $(,)*
            }
            unsized {
                $($param_name_lt:ident as ($param_alias_lt:expr): $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),*
                $(,)*
            }
        }
    ) => {
        request_ref! {
            $(#[$attr])*
            struct $struct_name for [$method_name]
            ($($const_param_name => $const_param_value),*) ->
            $response_type []
            {
                sized { $($param_name as ($param_alias): $param_type = $param_value => {$($value)*}),* }
                unsized { $($param_name_lt as ($param_alias_lt): $param_type_lt = $param_value_lt => {$($value_lt)*}),* }
            }
        }
    };

    // Struct with sized fields, permissions and aliases
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty [$($permission:ident),*]
        {
            sized {
                $($param_name:ident as ($param_alias:expr): $param_type:ty = $param_value:tt => {$($value:tt)*}),*
                $(,)*
            }
            unsized {
                $($param_name_lt:ident as ($param_alias_lt:expr): $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),*
                $(,)*
            }
        }
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        $(#[$attr])*
        pub struct $struct_name<'a> {
            $($param_name: $param_type,)*
            $($param_name_lt: &'a $param_type_lt,)*
        }

        impl<'a> ::api::Request for $struct_name<'a> {
            request_trait_impl! {
                [$method_name]
                ($($const_param_name => $const_param_value),*)
                -> $response_type [$($permission),*]
                {
                    $($param_name as $param_alias => {$($value)*},)*
                    $($param_name_lt as $param_alias_lt => {$($value_lt)*},)*
                }
            }
        }

        impl<'a> $struct_name<'a> {
            request_builder_impl! {
                $struct_name
                {
                    $($param_name: $param_type = $param_value,)*
                    $($param_name_lt: &'a $param_type_lt = $param_value_lt,)*
                }
            }
        }
    };
}

macro_rules! enum_str {
    (
        $(#[$attr:meta])*
        $name:ident {
            $($variant:ident = $value:expr),+
            $(,)*
        }
    ) => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        $(#[$attr])*
        pub enum $name {
            $($variant),+
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match *self {
                    $($name::$variant => $value),+
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<$name, ()> {
                match s {
                    $($value => Ok($name::$variant)),+,
                    _ => Err(()),
                }
            }
        }

        impl ::serde::de::Deserialize for $name {
            fn deserialize<D: ::serde::de::Deserializer>(d: &mut D) -> Result<$name, D::Error> {
                struct TempVisitor;

                impl ::serde::de::Visitor for TempVisitor {
                    type Value = $name;
                    fn visit_str<E: ::serde::de::Error>(&mut self, value: &str) -> Result<$name, E> {
                        match ::std::str::FromStr::from_str(value) {
                            Ok(temp_value) => Ok(temp_value),
                            _ => Err(::serde::de::Error::syntax("unexpected value")),
                        }
                    }
                }

                d.visit(TempVisitor)
            }
        }
    };
}
