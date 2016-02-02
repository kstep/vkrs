use super::users::{UserOptionField};
use super::api::{Collection, LikesCount};
use serde::de;

#[cfg(feature = "nightly")]
include!("wall.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/wall.rs"));

request_ref! {
    struct Get for ["wall.get"](v => 5.44, extended => 0) -> Collection<WallPost> {
        sized {
            owner_id: i64 = () => {},
            filter: Filter = (Filter::All) => {AsRef},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            domain: str = ("") => {=},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PostId(u64);

impl de::Deserialize for PostId {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<PostId, D::Error> {
        de::Deserialize::deserialize(d).map(PostId)
    }
}

request_ref! {
    struct Post for ["wall.post"](v => 5.44) -> PostId [Wall] {
        sized {
            owner_id: i64 = () => {},
            friend_only: bool = (true) => {bool},
            from_group: bool = (false) => {bool},
            signed: bool = (false) => {bool},
            publish_date: u64 = (0) => {},
            lat: f32 = () => {},
            long: f32 = () => {},
            place_id: u64 = () => {},
            post_id: u64 = () => {},
        }
        unsized {
            message: str = ("") => {=},
            services: str = ("") => {=},
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Filter {
    Owner,
    Others,
    All,
    Suggests,
}

impl AsRef<str> for Filter {
    fn as_ref(&self) -> &str {
        use self::Filter::*;
        match *self {
            Owner => "owner",
            Others => "others",
            All => "all",
            Suggests => "suggests",
        }
    }
}
