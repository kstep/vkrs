#![feature(custom_derive, plugin)]
#![feature(associated_consts)]
#![plugin(serde_macros)]
#![deny(unused_imports)]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

pub static API_URL: &'static str = "https://api.vk.com/method";

pub mod api;
pub mod auth;
pub mod audio;

#[test]
fn it_works() {
}
