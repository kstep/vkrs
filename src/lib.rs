#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate hyper;

pub static API_URL: &'static str = "https://api.vk.com/method";

pub mod auth;
pub use auth::*;

#[test]
fn it_works() {
}
