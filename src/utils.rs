#[cfg(feature = "nightly")]
include!("utils.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/utils.rs"));

request_lt! {
    struct CheckLink for ["utils.checkLink"](v => 5.44) -> LinkStatus {
        url: str = ("") => {=},
    }
}

request_lt! {
    struct ResolveScreenName for ["utils.resolveScreenName"](v => 5.44) -> Option<ScreenNameInfo> {
        screen_name: str = ("") => {=},
    }
}

request! {
    struct GetServerTime for ["users.getServerTime"](v => 5.44) -> u64;
}
