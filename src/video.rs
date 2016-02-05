use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use super::api::{Bool, Collection, Date, Duration, FullId, Id, LikesCount, OwnerId, Sort};

#[cfg(feature = "unstable")]
include!("video.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

request_ref! {
    #[derive(Eq, Copy)]
    struct Get for ["video.get"](v => 5.44) -> Collection<Video> {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            videos: [FullId] = (&[][..]) => { |value|
                &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
            }
        }
    }
}

request_ref! {
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
            filters: [Filter] = (&[][..]) => {AsRef<Vec>},
            q: str = ("") => {=},
        }
    }
}

enum_str! { Filter {
    YouTube = "youtube",
    Vimeo = "vimeo",
    Short = "short",
    Long = "long",
}}
