use super::api::{Collection};

request! {
    struct Search for ["photos.search"](q: String {AsRef}): Collection<Photo> [v => "5.37"] {
        lat: f32 [] {},
        long: f32 [] {},
        start_time: u64 [] {},
        end_time: u64 [] {},
        sort: Sort [Sort::Popularity] {AsRef},
        offset: usize [0] {},
        count: usize [30] {},
        radius: u16 [5000] {},
    }
}

#[cfg(feature = "nightly")]
include!("photos.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/photos.rs"));

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
