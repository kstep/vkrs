use api::Request;

#[cfg(feature = "nightly")]
include!("utils.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/utils.rs"));

request_lt! {
    struct CheckLink for ["utils.checkLink"](v => 5.44) -> LinkStatus {
        sized {
        }
        unsized {
            url: str = ("") => {=},
        }
    }
}

request_lt! {
    struct ResolveScreenName for ["utils.resolveScreenName"](v => 5.44) -> Option<ScreenNameInfo> {
        sized {
        }
        unsized {
            screen_name: str = ("") => {=},
        }
    }
}

pub struct GetServerTime;

impl Request for GetServerTime {
    type Response = u64;
    fn method_name() -> &'static str {
        "utils.getServerTime"
    }
    fn to_query_string(&self) -> String {
        "v=5.44".to_owned()
    }
}
