use std::convert::AsRef;

use hyper::client::Client as HttpClient;
use oauth2::provider::Provider;
use oauth2::client::response::{FromResponse, ParseError};
use oauth2::token::{Lifetime, Token};
use chrono::{DateTime, Duration, NaiveDateTime, UTC};
use rustc_serialize::json::Json;
use serde::{de, ser};
use super::api::Id;

pub use oauth2::ClientError as OAuthError;

#[derive(Debug, PartialEq, Eq)]
pub struct AccessTokenLifetime {
    expires: DateTime<UTC>,
}

impl de::Deserialize for AccessTokenLifetime {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<AccessTokenLifetime, D::Error> {
        de::Deserialize::deserialize(d)
            .map(|ts| AccessTokenLifetime { expires: DateTime::from_utc(NaiveDateTime::from_timestamp(ts, 0), UTC) })
    }
}

impl ser::Serialize for AccessTokenLifetime {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        ser::Serialize::serialize(&self.expires.timestamp(), s)
    }
}

#[cfg(feature = "unstable")]
include!("auth.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/auth.rs"));

impl FromResponse for AccessTokenLifetime {
    fn from_response(json: &Json) -> Result<AccessTokenLifetime, ParseError> {
        json.find("expires_in")
            .and_then(Json::as_i64)
            .map(|expires_in| AccessTokenLifetime { expires: UTC::now() + Duration::seconds(expires_in) })
            .ok_or(ParseError::ExpectedFieldType("expires_in", "i64"))
    }
}

impl FromResponse for AccessToken {
    fn from_response(json: &Json) -> Result<AccessToken, ParseError> {
        Ok(AccessToken {
            email: json.find("email").and_then(Json::as_string).map(ToOwned::to_owned),
            user_id: try!(json.find("user_id")
                              .and_then(Json::as_u64)
                              .ok_or(ParseError::ExpectedFieldType("user_id", "u64"))),
            access_token: try!(json.find("access_token")
                                   .and_then(Json::as_string)
                                   .map(ToOwned::to_owned)
                                   .ok_or(ParseError::ExpectedFieldType("access_token", "string"))),
            lifetime: try!(AccessTokenLifetime::from_response(json)),
        })
    }
}

impl Lifetime for AccessTokenLifetime {
    fn expired(&self) -> bool {
        self.expires <= UTC::now()
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
        OAuth(::oauth2::client::Client::<Auth>::new(key, secret, Some(String::from(OAUTH_DEFAULT_REDIRECT_URI))), client)
    }
    pub fn auth_uri<T: Into<Permissions>>(&self, scope: T) -> Result<String, OAuthError> {
        let scope: String = scope.into().into();
        self.0.auth_uri(Some(&scope), None)
    }
    pub fn request_token(&self, code: &str) -> Result<AccessToken, OAuthError> {
        self.0.request_token(self.1, code)
    }
}

pub struct Auth;
impl Provider for Auth {
    type Lifetime = AccessTokenLifetime;
    type Token = AccessToken;
    fn auth_uri() -> &'static str {
        "https://oauth.vk.com/authorize"
    }
    fn token_uri() -> &'static str {
        "https://oauth.vk.com/access_token"
    }
    fn credentials_in_body() -> bool {
        true
    }
}

pub static OAUTH_DEFAULT_REDIRECT_URI: &'static str = "https://oauth.vk.com/blank.html";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    Offline = 0,
    NoHttps = -1,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct Permissions(i32);
impl From<i32> for Permissions {
    fn from(n: i32) -> Permissions {
        Permissions(n & 0x5ebdff)
    }
}

impl Into<String> for Permissions {
    fn into(self) -> String {
        use self::Permission::*;
        let Permissions(n) = self;
        [Notify,
         Friends,
         Photos,
         Audio,
         Video,
         Docs,
         Notes,
         Pages,
         Menu,
         Status,
         Offers,
         Questions,
         Wall,
         Groups,
         Messages,
         Email,
         Notifications,
         Stats,
         Ads]
            .iter()
            .map(|&mask| mask)
            .filter(|&mask| mask as i32 & n != 0)
            .map(|mask| mask.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl From<Permission> for Permissions {
    fn from(perm: Permission) -> Permissions {
        Permissions(perm as i32)
    }
}

impl<'a> From<&'a [Permission]> for Permissions {
    fn from(vec: &[Permission]) -> Permissions {
        Permissions(vec.into_iter()
                       .map(|&mask| mask as i32)
                       .fold(0, |a, x| a + x))
    }
}

impl Into<Vec<Permission>> for Permissions {
    fn into(self) -> Vec<Permission> {
        use self::Permission::*;
        let Permissions(n) = self;
        [Notify,
         Friends,
         Photos,
         Audio,
         Video,
         Docs,
         Notes,
         Pages,
         Menu,
         Status,
         Offers,
         Questions,
         Wall,
         Groups,
         Messages,
         Email,
         Notifications,
         Stats,
         Ads]
            .iter()
            .map(|&mask| mask)
            .filter(|&mask| mask as i32 & n != 0)
            .collect()
    }
}

impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
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
            Offline => "offline",
            NoHttps => "nohttps",
        }
    }
}
