use auth::Permissions;
use api::{Id, Bool};
use users::{User, UserOptionField};

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

request! {
    #[derive(Eq, Copy)]
    struct SetOnline for ["account.setOnline"](v => 5.44) -> Bool {
        voip: bool = () => {bool},
    }
}

request! {
    struct SetOffline for ["account.setOffline"](v => 5.44) -> Bool;
}

request_ref! {
    #[derive(Eq, Copy)]
    struct LookupContacts for ["account.lookupContacts"](v => 5.44) -> Contacts [Friends] {
        sized {
            service: Service = (Service::Email) => {AsRef},
            return_all: bool = () => {bool},
        }
        unsized {
            contacts: str = ("") =>  {=},
            mycontact: str = ("") => {=},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

enum_str! { Service {
    Email = "email",
    Phone = "phone",
    Twitter = "twitter",
    Facebook = "facebook",
    Odnoklassniki = "odnoklassniki",
    Instagram = "instagram",
    Google = "google",
}}

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
