use auth::Permissions;
use api::{Bool, Id};
use users::{User, UserOptionField};
use serde_json::value::Value;
use serde_json::ser::to_string as json_to_string;

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

request_ref! {
    struct RegisterDevice for ["account.registerDevice"](v => 5.44) -> Bool [Messages] {
        sized {
            device_year: u16 = () => {},
            settings: Option<Value> = () => { |value|
                match value.as_ref().map(json_to_string) {
                    Some(Ok(ref value)) => &value,
                    _ => "",
                }
            },
            sandbox: bool = () => {bool},
        }
        unsized {
            token: str = ("") => {=},
            device_model: str = ("") => {=},
            device_id: str = ("") => {=},
            system_version: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct UnregisterDevice for ["account.unregisterDevice"](v => 5.44) -> Bool [Messages] {
        sized {
            sandbox: bool = () => {bool},
        }
        unsized {
            device_id: str = ("") => {=},
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
