use std::borrow::{Cow, Borrow};
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use hyper::Url;
use hyper::client::IntoUrl;
use url::{ParseError as UrlError};
use super::api::{Request, Response, Collection, Sort};

#[cfg(feature = "nightly")]
include!("video.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

impl Response for Video {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Get<'a> {
    owner_id: Option<i64>,
    videos: Cow<'a, [(i64, u64)]>,
    album_id: Option<u64>,
    offset: usize,
    count: usize,
}

impl<'a> Get<'a> {
    pub fn new() -> Get<'a> {
        Get {
            owner_id: None,
            album_id: None,
            videos: Cow::Borrowed(&[][..]),
            offset: 0,
            count: 100,
        }
    }
    pub fn videos<T: Into<Cow<'a, [(i64, u64)]>>>(&mut self, videos: T) -> &mut Get<'a> {
        self.videos = videos.into();
        self
    }
    pub fn owner_id(&mut self, value: i64) -> &mut Get<'a> {
        self.owner_id = Some(value);
        self
    }
    pub fn album_id(&mut self, value: u64) -> &mut Get<'a> {
        self.album_id = Some(value);
        self
    }
    pub fn count(&mut self, count: usize) -> &mut Get<'a> {
        self.count = count;
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Get<'a> {
        self.offset = offset;
        self
    }
}

impl<'a> Request<'a> for Get<'a> {
    type Response = Collection<Video>;
    fn method_name() -> &'static str { "video.get" }
}

impl<'a> IntoUrl for &'a Get<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(Get::base_url(qs![
            owner_id => self.owner_id.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or(""),
            videos => &*self.videos.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(","),
            album_id => self.album_id.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or(""),
            //extended => if self.extended {"1"} else {"0"},
            offset => &*self.offset.to_string(),
            count => &*self.count.to_string(),
            v => "5.44",
        ]))
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
     count: usize, // 0...300, def 30
}

impl<'a> Search<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(query: T) -> Search<'a> {
        Search {
            q: query.into(),
            hd: false,
            adult: false,
            filters: Cow::Borrowed(&[][..]),
            shorter: None,
            longer: None,
            sort: Sort::Popularity,
            search_own: false,
            offset: 0,
            count: 30,
        }
    }

    pub fn filters<T: Into<Cow<'a, [Filter]>>>(&mut self, filters: T) -> &mut Search<'a> {
        self.filters = filters.into();
        self
    }

    pub fn shorter(&mut self, value: usize) -> &mut Search<'a> {
        self.shorter = Some(value);
        self
    }

    pub fn longer(&mut self, value: usize) -> &mut Search<'a> {
        self.longer = Some(value);
        self
    }

    pub fn adult(&mut self, adult: bool) -> &mut Search<'a> {
        self.adult = adult;
        self
    }

    pub fn hd(&mut self, hd: bool) -> &mut Search<'a> {
        self.hd = hd;
        self
    }

    pub fn search_own(&mut self, search_own: bool) -> &mut Search<'a> {
        self.search_own = search_own;
        self
    }

    pub fn count(&mut self, count: usize) -> &mut Search<'a> {
        self.count = count;
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Search<'a> {
        self.offset = offset;
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Search<'a> {
        self.sort = sort;
        self
    }
}

impl<'a> Request<'a> for Search<'a> {
    type Response = Collection<Video>;
    fn method_name() -> &'static str { "video.search" }
}

impl<'a> IntoUrl for &'a Search<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(Search::base_url(qs![
            q => self.q.borrow(),
            sort => self.sort.as_ref(),
            search_own => if self.search_own {"1"} else {"0"},
            hd => if self.hd {"1"} else {"0"},
            adult => if self.adult {"1"} else {"0"},
            filters => &*self.filters.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(","),
            shorter => self.shorter.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or(""),
            longer => self.longer.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or(""),
            offset => &*self.offset.to_string(),
            count => &*self.count.to_string(),
            v => "5.44",
        ]))
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

