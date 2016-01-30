use std::borrow::{Borrow};
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use super::api::{Collection, Sort, Likes};

#[cfg(feature = "nightly")]
include!("video.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

request! {
    #[derive(Eq)]
    struct Get for ["video.get"](): Collection<Video> [v => "5.44"] {
        owner_id: Option<i64> [None] {Option},
        videos: Vec<(i64, u64)> [Vec::new()] { |value|
            &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
        },
        album_id: Option<u64> [None] {Option},
        offset: usize [0] {},
        count: usize [30] {},
    }
}

request! {
    #[derive(Eq)]
    struct Search for ["video.search"](q: String {AsRef}): Collection<Video> [v => "5.44"] {
        sort: Sort [Sort::Popularity] {AsRef},
        hd: bool [false] {bool},
        adult: bool [false] {bool},
        filters: Vec<Filter> [Vec::new()] { |value|
            &*value.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(",")
        },
        search_own: bool [false] {bool},
        longer: Option<usize> [None] {Option},
        shorter: Option<usize> [None] {Option},
        offset: usize [0] {},
        count: usize [30] {}, // 0...300, def 30
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

