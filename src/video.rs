use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use super::api::{Collection, Sort, Likes};

#[cfg(feature = "nightly")]
include!("video.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

request_lt! {
    #[derive(Eq, Copy)]
    struct Get for ["video.get"](v => 5.44) -> Collection<Video> {
        sized {
            owner_id: Option<i64> = () => {Option},
            album_id: Option<u64> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            videos: [(i64, u64)] = (&[][..]) => { |value|
                &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
            }
        }
    }
}

request_lt! {
    #[derive(Eq, Copy)]
    struct Search for ["video.search"](v => 5.44) -> Collection<Video> {
        sized {
            sort: Sort = (Sort::Popularity) => {AsRef},
            hd: bool = () => {bool},
            adult: bool = () => {bool},
            search_own: bool = () => {bool},
            longer: Option<usize> = () => {Option},
            shorter: Option<usize> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            filters: [Filter] = (&[][..]) => { |value|
                &*value.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(",")
            },
            q: str = ("") => {=},
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    YouTube,
    Vimeo,
    Short,
    Long,
}

impl AsRef<str> for Filter {
    fn as_ref(&self) -> &str {
        use self::Filter::*;
        match *self {
            YouTube => "youtube",
            Vimeo => "vimeo",
            Short => "short",
            Long => "long",
        }
    }
}

