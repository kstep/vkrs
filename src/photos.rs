use std::borrow::{Cow, Borrow};

use hyper::Url;
use hyper::client::IntoUrl;
use url::{ParseError as UrlError};

use super::api::{Request, Response, Collection};

#[derive(Debug, PartialEq, Clone)]
pub struct Search<'a> {
    q: Cow<'a, str>,
    lat: f32,
    long: f32,
    start_time: u64,
    end_time: u64,
    sort: Sort,
    offset: usize,
    count: usize,
    radius: u16,
}

impl<'a> Request<'a> for Search<'a> {
    type Response = Collection<Photo>;
    fn method_name() -> &'static str { "photos.search" }
}

impl<'a> Search<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(query: T) -> Search<'a> {
        Search {
            q: query.into(),
            lat: 0.0,
            long: 0.0,
            start_time: 0,
            end_time: 0,
            sort: Sort::Popularity,
            offset: 0,
            count: 100,
            radius: 5000,
        }
    }

    pub fn latitude(&mut self, lat: f32) -> &mut Search<'a> {
        self.lat = lat;
        self
    }
    pub fn longitude(&mut self, long: f32) -> &mut Search<'a> {
        self.long = long;
        self
    }
    pub fn start_time(&mut self, start_time: u64) -> &mut Search<'a> {
        self.start_time = start_time;
        self
    }
    pub fn end_time(&mut self, end_time: u64) -> &mut Search<'a> {
        self.end_time = end_time;
        self
    }
    pub fn sort(&mut self, sort: Sort) -> &mut Search<'a> {
        self.sort = sort;
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
    pub fn radius(&mut self, radius: u16) -> &mut Search<'a> {
        self.radius = radius;
        self
    }
}

impl<'a> IntoUrl for &'a Search<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(Search::base_url(qs![
            q => self.q.borrow(),
            start_time => &*self.start_time.to_string(),
            end_time => &*self.end_time.to_string(),
            lat => &*self.lat.to_string(),
            long => &*self.long.to_string(),
            radius => &*self.radius.to_string(),
            sort => self.sort.as_ref(),
            offset => &*self.offset.to_string(),
            count => &*self.count.to_string(),
            v => "5.37",
        ]))
    }
}

#[cfg(feature = "nightly")]
include!("photos.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/photos.rs"));

impl Response for Photo {}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Sort {
    DateAdded = 0,
    Popularity = 1,
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        use self::Sort::*;
        match *self {
            DateAdded => "0",
            Popularity => "1",
        }
    }
}

pub enum ReportReason {
    Spam = 0,
    ChildPorn = 1,
    Extremism = 2,
    Violence = 3,
    Drugs = 4,
    AdultOnly = 5,
    Offence = 6,
}
