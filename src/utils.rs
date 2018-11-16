use api::{Id, Timestamp};

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct LinkInfo {
    pub status: LinkStatus,
    pub link: String, // URL
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct ScreenNameInfo {
    #[serde(rename="type")]
    pub kind: String,
    pub object_id: Id,
}


enum_str! { LinkStatus {
    NotBanned = "not_banned",
    Banned = "banned",
    Processing = "processing"
}}

request_ref! {
    #[derive(Copy, Eq)]
    struct CheckLink for ["utils.checkLink"](v => 5.44) -> LinkInfo {
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
    struct GetServerTime for ["utils.getServerTime"](v => 5.44) -> Timestamp;
}
