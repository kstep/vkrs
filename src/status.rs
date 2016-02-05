use std::borrow::Borrow;
use audio::Audio;
use api::Bool;

#[cfg(feature = "unstable")]
include!("status.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/status.rs"));

request! {
    #[derive(Copy, Eq)]
    struct Get for ["status.get"](v => 5.44) -> Status [Status] {
        user_id: Option<i64> = () => {Option},
        group_id: Option<i64> = () => {Option},
    }
}

request_ref! {
    #[derive(Copy, Eq)]
    struct Set for ["status.set"](v => 5.44) -> Bool [Status] {
        sized {
            group_id: Option<i64> = () => {Option},
        }
        unsized {
            text: str = ("") => {=},
        }
    }
}
