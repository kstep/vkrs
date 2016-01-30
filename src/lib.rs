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

pub use api::{Result as VkResult, Error as ClientError, VkError, VkErrorCode};
pub use api::{Client, Collection, Sort};
pub use auth::{Permission, OAuthError, AccessToken};
pub use audio::{Audio, Lyrics, Genre, Album};
pub use audio::{Search as AudioSearch, Get as AudioGet, GetById as AudioGetById, GetLyrics as AudioGetLyrics};
pub use audio::{GetRecommendations as AudioGetRecommendations, GetCount as AudioGetCount, GetAlbums as AudioGetAlbums};
pub use audio::{GetPopular as AudioGetPopular};
pub use video::{Video};
pub use video::{Search as VideoSearch, Get as VideoGet};
pub use photos::{Photo, Search as PhotosSearch};

pub use hyper::client::IntoUrl;
