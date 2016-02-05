use auth::Permissions;
use api::{Id, Bool};

#[cfg(feature = "unstable")]
include!("account.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/account.rs"));

request! {
    #[derive(Eq, Copy)]
    struct GetAppPermissions for ["account.getAppPermissions"](v => 5.44) -> Permissions {
        user_id: Id = () => {}
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct GetCounters for ["account.getCounters"](v => 5.44) -> Counters {
        filter: [Filter] = (&[][..]) => {AsRef<Vec>},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct SetNameInMenu for ["account.setNameInMenu"](v => 5.44) -> Bool {
        sized {
            user_id: Id = () => {},
        }
        unsized {
            name: str = ("") => {=},
        }
    }
}

enum_str! { Filter {
    Friends = "friends",
    Messages = "messages",
    Photos = "photos",
    Videos = "videos",
    Notes = "notes",
    Gifts = "gifts",
    Events = "events",
    Groups = "groups",
    Sdk = "sdk",
}}
