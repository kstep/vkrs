#![macro_use]

macro_rules! qs {
    ($($name:ident => $value:expr),+ $(),*) => {
        ::url::form_urlencoded::serialize([
            $((stringify!($name), $value)),*
        ].into_iter().filter(|&&(_, v)| !v.is_empty()))
    }
}

macro_rules! expand_value_expr {
    ($this:ident; $param_name:ident;) => {
        expand_value_expr!($this; $param_name; |value|
                           &*value.to_string())
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
    () => { Default::default() };
    ($value:expr) => { $value };
}

macro_rules! request {
    (
        $(#[$attr:meta])*
        struct $struct_name:ident for [$method_name:expr] (
            $($def_param_name:ident: $def_param_type:ty {$($def_value:tt)*}),*
            ): $response_type:ty
        [$($const_param_name:ident => $const_param_value:expr),*] {
            $($param_name:ident: $param_type:ty [$($param_value:expr)*] {$($value:tt)*}),*
            $(,)*
        }
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        $(#[$attr])*
        pub struct $struct_name {
            $($param_name: $param_type,)*
            $($def_param_name: $def_param_type,)*
        }

        impl ::api::Request for $struct_name {
            type Response = $response_type;
            fn method_name() -> &'static str { $method_name }
            fn to_query_string(&self) -> String {
                qs![
                    $($param_name => expand_value_expr!(self; $param_name; $($value)*),)*
                    $($const_param_name => $const_param_value,)*
                    $($def_param_name => expand_value_expr!(self; $def_param_name; $($def_value)*),)*
                ]
            }
        }

        impl $struct_name {
            pub fn new($($def_param_name: $def_param_type),*) -> $struct_name {
                $struct_name {
                    $($def_param_name: $def_param_name,)*
                    $($param_name: expand_init_expr!($($param_value)*),)*
                }
            }

            $(pub fn $param_name(&mut self, value: $param_type) -> &mut Self {
                self.$param_name = value;
                self
            })*
        }
    };
}
