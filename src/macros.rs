#![macro_use]

macro_rules! qs {
    ($($name:ident => $value:expr),+ $(,)*) => {
        ::url::form_urlencoded::serialize([
            $((stringify!($name), $value)),*
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
        [$method_name:expr]($($const_param_name:ident => $const_param_value:expr),*) -> $response_type:ty
        {
            $($param_name:ident => {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        type Response = $response_type;
        fn method_name() -> &'static str { $method_name }
        fn to_query_string(&self) -> String {
            qs![
                $($param_name => expand_value_expr!(self; $param_name; $($value)*),)*
                $($const_param_name => concat!($const_param_value),)*
            ]
        }
    }
}

macro_rules! request {
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
        #[derive(Debug, PartialEq, Clone)]
        $(#[$attr])*
        pub struct $struct_name {
            $($param_name: $param_type,)*
        }

        impl ::api::Request for $struct_name {
            request_trait_impl! {
                [$method_name]
                ($($const_param_name => $const_param_value),*)
                -> $response_type
                {
                    $($param_name => {$($value)*},)*
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

macro_rules! request_lt {
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr]
        ($($const_param_name:ident => $const_param_value:expr),*) ->
        $response_type:ty
        {
            sized {$($param_name:ident: $param_type:ty = $param_value:tt => {$($value:tt)*}),* $(,)*}
            unsized {$($param_name_lt:ident: $param_type_lt:ty = $param_value_lt:tt => {$($value_lt:tt)*}),* $(,)*}
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
                -> $response_type
                {
                    $($param_name => {$($value)*},)*
                    $($param_name_lt => {$($value_lt)*},)*
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

