use std::borrow::Borrow;
use audio::Audio;

#[cfg(feature = "nightly")]
include!("status.rs.in");

#[cfg(not(feature = "nightly"))]
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
    struct Set for ["status.set"](v => 5.44) -> u8 [Status] {
        sized {
            group_id: Option<i64> = () => {Option},
        }
        unsized {
            text: str = ("") => {=},
        }
    }
}
