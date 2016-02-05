use api::{Date, Id};

#[cfg(feature = "unstable")]
include!("utils.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/utils.rs"));

request_ref! {
    #[derive(Copy, Eq)]
    struct CheckLink for ["utils.checkLink"](v => 5.44) -> LinkStatus {
        url: str = ("") => {=},
    }
}

request_ref! {
    #[derive(Copy, Eq)]
    struct ResolveScreenName for ["utils.resolveScreenName"](v => 5.44) -> Option<ScreenNameInfo> {
        screen_name: str = ("") => {=},
    }
}

request! {
    struct GetServerTime for ["utils.getServerTime"](v => 5.44) -> Date;
}
