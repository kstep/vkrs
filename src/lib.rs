#![feature(custom_derive, plugin)]
#![feature(associated_consts)]
#![plugin(serde_macros)]
#![deny(unused_imports)]

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

pub mod api;
pub mod auth;
pub mod audio;
pub mod photos;

pub use api::{WithToken, VkResult, VkError, VkErrorCode, Collection};
pub use auth::{Permission, OAuth, AccessToken, AccessTokenResult};
pub use audio::{Audio, Search as AudioSearch, Get as AudioGet};
pub use photos::{Photo, Search as PhotosSearch};

pub use hyper::client::{Client, IntoUrl};
