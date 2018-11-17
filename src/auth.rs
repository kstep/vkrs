use url::Url;
use oauth2::provider::Provider;
use oauth2::client::response::{FromResponse, ParseError};
use oauth2::token::{Lifetime, Token};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use serde::{de, ser};
use api::{Id, Request};
use std::ops::BitOr;
use std::iter::FromIterator;
use std::str::FromStr;
use serde_json::Value as Json;

pub use oauth2::ClientError as OAuthError;
use api::HttpClient;

#[derive(Debug, PartialEq, Eq)]
pub struct AccessTokenLifetime {
    expires: Option<DateTime<Utc>>,
}

impl<'de> de::Deserialize<'de> for AccessTokenLifetime {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<AccessTokenLifetime, D::Error> {
        de::Deserialize::deserialize(d).map(|ts: Option<u64>| {
            AccessTokenLifetime { expires: ts.map(|ts| DateTime::from_utc(NaiveDateTime::from_timestamp(ts as i64, 0), Utc)) }
        })
    }
}

impl ser::Serialize for AccessTokenLifetime {
    fn serialize<S: ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        ser::Serialize::serialize(&self.expires.map(|ts| ts.timestamp()), s)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessToken {
    access_token: String,
    pub user_id: Id,
    pub email: Option<String>,
    lifetime: AccessTokenLifetime,
}


impl FromResponse for AccessTokenLifetime {
    fn from_response(json: &Json) -> Result<AccessTokenLifetime, ParseError> {
        json.pointer("expires_in")
            .and_then(Json::as_i64)
            .map(|expires_in| {
                AccessTokenLifetime { expires: if expires_in > 0 { Some(Utc::now() + Duration::seconds(expires_in)) } else { None } }
            })
            .ok_or_else(|| ParseError::ExpectedFieldType("expires_in", "i64"))
    }
}

impl FromResponse for AccessToken {
    fn from_response(json: &Json) -> Result<AccessToken, ParseError> {
        Ok(AccessToken {
            email: json.pointer("email").and_then(Json::as_str).map(ToOwned::to_owned),
            user_id: json.pointer("user_id")
                .and_then(Json::as_u64)
                .ok_or(ParseError::ExpectedFieldType("user_id", "u64"))?,
            access_token: json.pointer("access_token")
                .and_then(Json::as_str)
                .map(ToOwned::to_owned)
                .ok_or(ParseError::ExpectedFieldType("access_token", "string"))?,
            lifetime: AccessTokenLifetime::from_response(json)?,
        })
    }
}

impl Lifetime for AccessTokenLifetime {
    fn expired(&self) -> bool {
        self.expires.map_or(false, |e| e <= Utc::now())
    }
}

impl Token<AccessTokenLifetime> for AccessToken {
    fn access_token(&self) -> &str {
        &*self.access_token
    }
    fn scope(&self) -> Option<&str> {
        None
    }
    fn lifetime(&self) -> &AccessTokenLifetime {
        &self.lifetime
    }
}

impl AccessToken {
    pub fn expired(&self) -> bool {
        self.lifetime.expired()
    }
}

pub struct OAuth<'a>(::oauth2::client::Client<Auth>, &'a HttpClient);

impl<'a> OAuth<'a> {
    pub fn new(client: &'a HttpClient, key: String, secret: String) -> OAuth {
        OAuth(::oauth2::client::Client::new(Auth, key, secret, Some(String::from(OAUTH_DEFAULT_REDIRECT_URI))), client)
    }
    pub fn auth_uri<T: Into<Permissions>>(&self, scope: T) -> Url {
        let scope: String = scope.into().into();
        self.0.auth_uri(Some(&scope), None)
    }
    pub fn auth_uri_for<T: Request>(&self) -> Url {
        let scope = <T as Request>::permissions();
        self.auth_uri(scope)
    }
    pub fn request_token(&self, code: &str) -> Result<AccessToken, OAuthError> {
        self.0.request_token(self.1, code)
    }
}

pub struct Auth;
lazy_static! {
    static ref VK_OAUTH_AUTH_URL: Url = Url::parse("https://oauth.vk.com/authorize").unwrap();
    static ref VK_OAUTH_TOKEN_URL: Url = Url::parse("https://oauth.vk.com/access_token").unwrap();
}
impl Provider for Auth {
    type Lifetime = AccessTokenLifetime;
    type Token = AccessToken;
    fn auth_uri(&self) -> &Url {
        &VK_OAUTH_AUTH_URL
    }
    fn token_uri(&self) -> &Url {
        &VK_OAUTH_TOKEN_URL
    }
    fn credentials_in_body(&self) -> bool {
        true
    }
}

pub static OAUTH_DEFAULT_REDIRECT_URI: &'static str = "https://oauth.vk.com/blank.html";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(overflowing_literals)]
#[repr(i32)]
pub enum Permission {
    Notify = 1,
    Friends = 2,
    Photos = 4,
    Audio = 8,
    Video = 16,
    Docs = 131072,
    Notes = 2048,
    Pages = 128,
    Menu = 256,
    Status = 1024,
    Offers = 32,
    Questions = 64,
    Wall = 8192,
    Groups = 262144,
    Messages = 4096,
    Email = 4194304,
    Notifications = 524288,
    Stats = 1048576,
    Ads = 32768,
    Market = 134217728,
    Offline = 65536,
    NoHttps = 0x8000_0000 as i32, // unofficial
}

static PERMISSIONS: &'static [Permission] = &[Permission::Notify,
                                              Permission::Friends,
                                              Permission::Photos,
                                              Permission::Audio,
                                              Permission::Video,
                                              Permission::Docs,
                                              Permission::Notes,
                                              Permission::Pages,
                                              Permission::Menu,
                                              Permission::Status,
                                              Permission::Offers,
                                              Permission::Questions,
                                              Permission::Wall,
                                              Permission::Groups,
                                              Permission::Messages,
                                              Permission::Email,
                                              Permission::Notifications,
                                              Permission::Stats,
                                              Permission::Ads,
                                              Permission::Market,
                                              Permission::Offline,
                                              Permission::NoHttps];

impl Permission {
    pub fn variants() -> &'static [Permission] {
        PERMISSIONS
    }

    pub fn mask(&self) -> i32 {
        *self as i32
    }

    pub fn mask_all() -> i32 {
        0x5ebdff
    }

    pub fn to_str(&self) -> &'static str {
        use self::Permission::*;
        match *self {
            Notify => "notify",
            Friends => "friends",
            Photos => "photos",
            Audio => "audio",
            Video => "video",
            Docs => "docs",
            Notes => "notes",
            Pages => "pages",
            Menu => "menu",
            Status => "status",
            Offers => "offers",
            Questions => "questions",
            Wall => "wall",
            Groups => "groups",
            Messages => "messages",
            Email => "email",
            Notifications => "notifications",
            Stats => "stats",
            Ads => "ads",
            Market => "market",
            Offline => "offline",
            NoHttps => "nohttps",
        }
    }
}

impl FromStr for Permission {
    type Err = ();
    fn from_str(s: &str) -> Result<Permission, ()> {
        use self::Permission::*;
        Ok(match s {
            "notify" => Notify,
            "friends" => Friends,
            "photos" => Photos,
            "audio" => Audio,
            "video" => Video,
            "docs" => Docs,
            "notes" => Notes,
            "pages" => Pages,
            "menu" => Menu,
            "status" => Status,
            "offers" => Offers,
            "questions" => Questions,
            "wall" => Wall,
            "groups" => Groups,
            "messages" => Messages,
            "email" => Email,
            "notifications" => Notifications,
            "stats" => Stats,
            "ads" => Ads,
            "market" => Market,
            "offline" => Offline,
            "nohttps" => NoHttps,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct Permissions(i32);

impl Permissions {
    pub fn new(n: i32) -> Permissions {
        Permissions(n & Permission::mask_all())
    }
}

impl<'de> de::Deserialize<'de> for Permissions {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Permissions, D::Error> {
        de::Deserialize::deserialize(d).map(Permissions::new)
    }
}

impl FromStr for Permissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Permissions, ()> {
        s.split(',').map(str::trim).map(Permission::from_str).collect()
    }
}

impl From<Permission> for Permissions {
    fn from(perm: Permission) -> Permissions {
        Permissions(perm as i32)
    }
}

impl<'a, T: IntoIterator<Item = &'a Permission>> From<T> for Permissions {
    fn from(iter: T) -> Permissions {
        iter.into_iter().collect()
    }
}

impl FromIterator<i32> for Permissions {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Permissions {
        Permissions(iter.into_iter().fold(0, BitOr::bitor))
    }
}

impl FromIterator<Permission> for Permissions {
    fn from_iter<T: IntoIterator<Item = Permission>>(iter: T) -> Permissions {
        Permissions(iter.into_iter().map(|perm| perm as i32).fold(0, BitOr::bitor))
    }
}

impl<'a> FromIterator<&'a Permission> for Permissions {
    fn from_iter<T: IntoIterator<Item = &'a Permission>>(iter: T) -> Permissions {
        Permissions(iter.into_iter().map(|&perm| perm as i32).fold(0, BitOr::bitor))
    }
}

impl Into<String> for Permissions {
    fn into(self) -> String {
        Into::<Vec<&'static str>>::into(self).join(",")
    }
}

impl Into<Vec<Permission>> for Permissions {
    fn into(self) -> Vec<Permission> {
        let Permissions(n) = self;
        Permission::variants()
            .iter()
            .filter(|&&mask| mask as i32 & n != 0)
            .cloned()
            .collect()
    }
}

impl Into<Vec<&'static str>> for Permissions {
    fn into(self) -> Vec<&'static str> {
        let Permissions(n) = self;
        Permission::variants()
            .iter()
            .filter(|&&mask| mask as i32 & n != 0)
            .map(Permission::to_str)
            .collect()
    }
}
