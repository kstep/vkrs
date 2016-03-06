use std::borrow::Borrow;
use std::convert::AsRef;
use serde::de;
use api::{Bool, Collection, Id};

#[cfg(feature = "unstable")]
include!("users.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/users.rs"));

enum_str! { NameCase {
    Nominative = "nom",
    Genetive = "gen",
    Dative = "dat",
    Accusative = "acc",
    Instrumental = "ins",
    Ablative = "abl",
}}

enum_str! { UserOptionField {
    Verified = "verified",
    Blacklisted = "blacklisted",
    Sex = "sex",
    Birthdate = "bdate",
    City = "city",
    Country = "country",
    HomeTown = "home_town",
    Photo50 = "photo_50",
    Photo100 = "photo_100",
    Photo200Orig = "photo_200_orig",
    Photo200 = "photo_200",
    Photo400Orig = "photo_400_orig",
    PhotoMax = "photo_max",
    PhotoMaxOrig = "photo_max_orig",
    Online = "online",
    Lists = "lists",
    Domain = "domain",
    HasMobile = "has_mobile",
    Contacts = "contacts",
    Site = "site",
    Education = "education",
    Universities = "universities",
    Schools = "schools",
    Status = "status",
    LastSeen = "last_seen",
    FollowersCount = "followers_count",
    CommonCount = "common_count",
    Counters = "counters",
    Occupation = "occupation",

    Nickname = "nickname",
    Relatives = "relatives",
    Relation = "relation",
    Personal = "personal",
    Connections = "connections",
    Exports = "exports",
    WallComments = "wall_comments",
    Activities = "activities",
    Interests = "interests",
    Music = "music",
    Movies = "movies",
    TvShows = "tv",
    Books = "books",
    Games = "games",
    About = "about",
    Quotes = "quotes",
    CanPost = "canPost",
    CanSeeAllPosts = "can_see_all_posts",
    CanSeeAudio = "can_see_audio",
    CanWritePrivateMessage = "can_write_private_message",
    Timezone = "timezone",
    ScreenName = "screen_name",
}}

request_ref! {
    struct Get for ["users.get"](v => 5.44) -> Vec<User> {
        sized {
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
        }
        unsized {
            user_ids: [Id] = (&[][..]) => {Vec},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_ref! {
    struct Search for ["users.search"](v => 5.44) -> Collection<User> {
        sized {
            sort: Sort = (Sort::Rating) => {AsRef},

            city: Option<Id> = () => {Option},
            country: Option<Id> = () => {Option},

            university: Option<Id> = () => {Option},
            university_country: Option<Id> = () => {Option},
            university_faculty: Option<Id> = () => {Option},
            university_chair: Option<Id> = () => {Option},
            university_year: Option<u16> = () => {Option},

            sex: Sex = (Sex::Any) => {AsRef},
            status: Status = (Status::Unspecified) => {AsRef},

            age_from: Option<u16> = () => {Option},
            age_to: Option<u16> = () => {Option},
            birth_day: Option<u8> = () => {Option},
            birth_month: Option<u8> = () => {Option},
            birth_year: Option<u16> = () => {Option},

            online: bool = (false) => {bool},
            has_photo: bool = (false) => {bool},

            school: Option<Id> = () => {Option},
            school_country: Option<Id> = () => {Option},
            school_city: Option<Id> = () => {Option},
            school_class: Option<Id> = () => {Option},
            school_year: Option<u16> = () => {Option},

            group_id: Option<Id> = () => {Option},

            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            q: str = ("") => {=},
            hometown: str = ("") => {=},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
            religion: str = ("") => {=},
            interests: str = ("") => {=},
            company: str = ("") => {=},
            position: str = ("") => {=},
            from_list: str = ("") => {=},
        }
    }
}

request! {
    struct IsAppUser for ["users.isAppUser"](v => 5.44) -> Bool {
        user_id: Id = (0) => {}
    }
}

request_ref! {
    struct GetSubscriptions for ["users.getSubscriptions"](v => 5.44, extended => 1) -> Collection<User> {
        sized {
            user_id: Id = (0) => {},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_ref! {
    struct GetFollowers for ["users.getFollowers"](v => 5.44) -> Collection<User> {
        sized {
            user_id: Id = (0) => {},
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_ref! {
    struct Report for ["users.report"](v => 5.44) -> Bool {
        sized {
            user_id as ("user_id"): Id = (0) => {},
            kind as ("type"): ReportKind = (ReportKind::Spam) => {AsRef},
        }
        unsized {
            comment as ("comment"): str = ("") => {=},
        }
    }
}

// request_ref! {
// struct GetSubscriptionIds for ["users.getSubscriptions"](v => 5.44, extended => 0) -> Collection<i64> {
// sized {
// user_id: Id (0) => {},
// offset: usize (0) => {},
// count: usize (100) => {},
// }
// unsized {
// fields: str = ("") => {=},
// }
// }
// }

request_ref! {
    struct GetNearby for ["users.getNearby"](v => 5.44) -> Collection<User> {
        sized {
            latitude: f32 = () => {},
            longitude: f32 = () => {},
            accuracy: u16 = () => {},
            timeout: u16 = (7299) => {},
            radius: Radius = (Radius::R300) => {AsRef},
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
        }
        unsized {
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Radius {
    R300 = 1,
    R2400 = 2,
    R18000 = 3,
    R150000 = 4,
}

impl AsRef<str> for Radius {
    fn as_ref(&self) -> &str {
        use self::Radius::*;
        match *self {
            R300 => "1",
            R2400 => "2",
            R18000 => "3",
            R150000 => "4",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Sort {
    Rating = 0,
    DateRegistered = 1,
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        match *self {
            Sort::Rating => "0",
            Sort::DateRegistered => "1",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Sex {
    Any = 0,
    Female = 1,
    Male = 2,
}

impl AsRef<str> for Sex {
    fn as_ref(&self) -> &str {
        match *self {
            Sex::Any => "0",
            Sex::Female => "1",
            Sex::Male => "2",
        }
    }
}

impl de::Deserialize for Sex {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Sex, D::Error> {
        use self::Sex::*;
        de::Deserialize::deserialize(d).and_then(|value: u8| {
            match value {
                0 => Ok(Any),
                1 => Ok(Female),
                2 => Ok(Male),
                _ => Err(de::Error::invalid_value("integer value in range 0...2 expected")),
            }
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Status {
    Unspecified = 0,
    NotMarried = 1,
    InRelationship = 2,
    Engaged = 3,
    Married = 4,
    ItsComplicated = 5,
    ActiveSearch = 6,
    InLove = 7,
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        use self::Status::*;
        match *self {
            Unspecified => "0",
            NotMarried => "1",
            InRelationship => "2",
            Engaged => "3",
            Married => "4",
            ItsComplicated => "5",
            ActiveSearch => "6",
            InLove => "7",
        }
    }
}

impl de::Deserialize for Status {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Status, D::Error> {
        use self::Status::*;
        de::Deserialize::deserialize(d).and_then(|value: u8| {
            match value {
                0 => Ok(Unspecified),
                1 => Ok(NotMarried),
                2 => Ok(InRelationship),
                3 => Ok(Engaged),
                4 => Ok(Married),
                5 => Ok(ItsComplicated),
                6 => Ok(ActiveSearch),
                7 => Ok(InLove),
                _ => Err(de::Error::invalid_value("integer value in range 0...7 expected")),
            }
        })
    }
}

enum_str! { ReportKind {
    Porn = "porn",
    Spam = "spam",
    Insult = "insult",
    Ads = "advertisment",
}}
