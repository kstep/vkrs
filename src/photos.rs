use std::borrow::{Cow, Borrow};

use hyper::Url;
use hyper::client::IntoUrl;
use url::ParseError as UrlError;

use super::api::{WithToken, Request, Response, VK_METHOD_URL};


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
    token: Option<Cow<'a, str>>
}

impl<'a> Request<'a> for Search<'a> {
    const METHOD_NAME: &'static str = "photos.search";
}

impl<'a> WithToken<'a> for Search<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&'a mut self, token: T) -> &'a mut Search<'a> {
        self.token = Some(token.into());
        self
    }
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
            token: None
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
        let mut url = Url::parse(&*(VK_METHOD_URL.to_owned() + Search::METHOD_NAME)).unwrap();
        url.set_query_from_pairs([
                                 ("q", self.q.borrow()),
                                 ("start_time", &*self.start_time.to_string()),
                                 ("end_time", &*self.end_time.to_string()),
                                 ("lat", &*self.lat.to_string()),
                                 ("long", &*self.long.to_string()),
                                 ("radius", &*self.radius.to_string()),
                                 ("sort", self.sort.as_ref()),
                                 ("offset", &*self.offset.to_string()),
                                 ("count", &*self.count.to_string()),
                                 ("v", "5.37"),
                                 //("access_token", self.token.as_ref().unwrap().borrow())
                                 ].iter().cloned());
        Ok(url)
    }
}

#[derive(Debug, Deserialize)]
pub struct Photo {
    pub id: u64,
    pub album_id: i64,
    pub owner_id: i64,
    pub user_id: i64,
    pub photo_75: String,
    pub photo_130: String,
    pub photo_604: String,
    pub width: u16,
    pub height: u16,
    pub text: String,
    pub date: u64,
}

impl Response for Photo {}

#[derive(Debug)]
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
