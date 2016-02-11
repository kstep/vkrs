use std::borrow::Borrow;
use api::{Bool, Collection, Id, OwnerId};
use serde::de;
use std::fmt::Debug;
use std::str::FromStr;
use chrono::offset::local::Local;
pub use chrono::naive::date::NaiveDate;

#[cfg(feature = "unstable")]
include!("stats.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/stats.rs"));

request! {
    #[derive(Copy, Eq)]
    struct Get for ["stats.get"](v => 5.44) -> Collection<Period> {
        group_id: Option<Id> = () => {Option},
        app_id: Option<Id> = () => {Option},
        date_from: NaiveDate = (Local::today().naive_local()) => {},
        date_to: NaiveDate = (Local::today().succ().naive_local()) => {},
    }
}

request! {
    struct TrackVisitor for ["stats.trackVisitor"](v => 5.44) -> Bool;
}

request! {
    #[derive(Copy, Eq)]
    struct GetPostReach for ["stats.getPostReach"](v => 5.44) -> PostReach {
        owner_id: OwnerId = () => {},
        post_id: Id = () => {},
    }
}
