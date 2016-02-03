#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![deny(unused_imports)]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;
extern crate inth_oauth2 as oauth2;
extern crate rustc_serialize;
extern crate chrono;

mod macros;
pub mod api;
pub mod auth;
pub mod audio;
pub mod photos;
pub mod video;
pub mod users;
pub mod status;
pub mod wall;
pub mod utils;
pub mod execute;
