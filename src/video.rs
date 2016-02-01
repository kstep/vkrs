use std::borrow::{Borrow, Cow};
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use super::api::{Collection, Sort, Likes};

#[cfg(feature = "nightly")]
include!("video.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct Get<'a> {
    owner_id: Option<i64>,
    videos: Cow<'a, [(i64, u64)]>,
    album_id: Option<u64>,
    offset: usize,
    count: usize,
}
impl<'a> ::api::Request for Get<'a> {
    request_trait_impl! {
        ["video.get"](v => "5.44") -> Collection<Video> {
            owner_id => {Option},
            videos => { |value|
                &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
            },
            album_id => {Option},
            offset => {},
            count => {}
        }
    }
}
impl<'a> Get<'a> {
    request_builder_impl! {
        Get {
            owner_id: Option<i64> = (),
            videos: Cow<'a, [(i64, u64)]> = {Cow::Borrowed(&[][..])},
            album_id: Option<u64> = (),
            offset: usize = (0),
            count: usize = (30)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Search<'a> {
    q: Cow<'a, str>,
    sort: Sort,
    hd: bool,
    adult: bool,
    filters: Cow<'a, [Filter]>,
    search_own: bool,
    longer: Option<usize>,
    shorter: Option<usize>,
    offset: usize,
    count: usize,
}
impl<'a> Search<'a> { request_builder_impl! {
    Search {
        q: Cow<'a, str> = {Cow::Borrowed("")},
        sort: Sort = (Sort::Popularity),
        hd: bool = (),
        adult: bool = (),
        filters: Cow<'a, [Filter]> = {Cow::Borrowed(&[][..])},
        search_own: bool = (),
        longer: Option<usize> = (),
        shorter: Option<usize> = (),
        offset: usize = (0),
        count: usize = (30),
    }
}}
impl<'a> ::api::Request for Search<'a> { request_trait_impl! {
    ["video.search"](v => "5.44") -> Collection<Video> {
        q => {Borrow},
        sort => {AsRef},
        hd => {bool},
        adult => {bool},
        filters => { |value|
            &*value.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(",")
        },
        search_own => {bool},
        longer => {Option},
        shorter => {Option},
        offset => {},
        count => {},
    }
}}

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

