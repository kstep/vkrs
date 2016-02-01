use super::api::Collection;
use std::borrow::Borrow;
use audio::Audio;

#[cfg(feature = "nightly")]
include!("status.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/status.rs"));

request! {
    struct Get for ["status.get"](v => 5.44) -> Collection<Status> {
        user_id: Option<i64> = () => {Option},
        group_id: Option<i64> = () => {Option},
    }
}

request_lt! {
    struct Set for ["status.set"](v => 5.44) -> u8 {
        sized {
            group_id: Option<i64> = () => {Option},
        }
        unsized {
            text: str = ("") => {=},
        }
    }
}