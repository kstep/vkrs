use auth::Permissions;
use std::borrow::Borrow;
use api::{Bool, Collection, Duration, Id, Profile, Timestamp};
use users::{Sex, Status as Relation, User, UserOptionField};
use serde_json::value::Value;
use serde_json::ser::to_string as json_to_string;
use serde::de;

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

request_ref! {
    #[derive(Eq, Copy)]
    struct SetSilenceMode for ["account.setSilenceMode"](v => 5.44) -> Bool [Messages] {
        sized {
            time: Duration = () => {},
            chat_id: Id = () => {},
            user_id: Id = () => {},
            sound: Id = () => {}, // what's this?
        }
        unsized {
            device_id: str = ("") => {=},
        }
    }
}

request_ref! {
    struct GetPushSettings for ["account.getPushSettings"](v => 5.44) -> PushSettings [Messages] {
        device_id: str = ("") => {=},
    }
}

request_ref! {
    struct SetPushSettings for ["account.setPushSettings"](v => 5.44) -> Bool [Messages] {
        sized {
            settings: Option<Value> = () => { |value|
                match value.as_ref().map(json_to_string) {
                    Some(Ok(ref value)) => &**value,
                    _ => "",
                }
            }
        }
        unsized {
            key: str = ("") => {=},
            value: str = ("") => {=},
            device_id: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetActiveOffers for ["account.getActiveOffers"](v => 5.44) -> Collection<Offer> {
        offset: usize = (0) => {},
        count: usize = (100) => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct BanUser for ["account.banUser"](v => 5.44) -> Bool {
        user_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct UnbanUser for ["account.unbanUser"](v => 5.44) -> Bool {
        user_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetBanned for ["account.getBanned"](v => 5.44) -> Collection<Profile> {
        offset: usize = (0) => {},
        count: usize = (100) => {},
    }
}

request! {
    struct GetInfo for ["account.getInfo"](v => 5.44) -> Info;
    // This method actually accepts `fields` argument with
    // fields list to return, but I will have to set make all
    // fields in Info optional. I don't see any sense to give
    // user the freedom to change response format here.
}

request! {
    struct SetInfo for ["account.setInfo"](v => 5.44) -> Bool {
        intro: u32 = () => {},
        own_posts_default: bool = () => {bool},
        no_wall_replies: bool = () => {bool},
    }
}

request_ref! {
    struct ChangePassword for ["account.changePassword"](v => 5.44) -> ChangedToken {
        restore_sid: str = ("") => {=},
        change_password_hash: str = ("") => {=},
        old_password: str = ("") => {=},
        new_password: str = ("") => {=},
    }
}

request! {
    struct GetProfileInfo for ["account.getProfileInfo"](v => 5.44) -> ProfileInfo;
}

request_ref! {
    struct SetProfileInfo for ["account.setProfileInfo"](v => 5.44) -> ChangedProfileInfo {
        sized {
            cancel_request_id: Option<Id> = () => {Option},
            sex: Option<Sex> = () => {AsRef<Option>},
            relation: Option<Relation> = () => {AsRef<Option>},
            relation_partner_id: Option<Id> = () => {Option},
            bdate_visibility: Option<BirthdateVisibility> = () => {AsRef<Option>},
            country_id: Option<Id> = () => {Option},
            city_id: Option<Id> = () => {Option},
        }
        unsized {
            first_name: str = ("") => {=},
            last_name: str = ("") => {=},
            maiden_name: str = ("") => {=},
            screen_name: str = ("") => {=},
            bdate: str = ("") => {=},
            home_town: str = ("") => {=},
            status: str = ("") => {=},
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

enum_str! { NameChangeStatus {
    Processing = "processing",
    Declined = "declined",
}}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BirthdateVisibility {
    Hide = 0,
    ShowYMD = 1,
    ShowMD = 2,
}

impl de::Deserialize for BirthdateVisibility {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<BirthdateVisibility, D::Error> {
        use self::BirthdateVisibility::*;
        de::Deserialize::deserialize(d).and_then(|value: u8| {
            match value {
                0 => Ok(Hide),
                1 => Ok(ShowYMD),
                2 => Ok(ShowMD),
                _ => Err(de::Error::syntax("integer value in range 0...2 expected")),
            }
        })
    }
}

impl AsRef<str> for BirthdateVisibility {
    fn as_ref(&self) -> &str {
        match *self {
            BirthdateVisibility::Hide => "0",
            BirthdateVisibility::ShowYMD => "1",
            BirthdateVisibility::ShowMD => "2",
        }
    }
}
