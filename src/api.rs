use std::fmt;
use std::ops::Deref;
use std::marker::PhantomData;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use serde::de;
use serde_json::{self, Error as JsonError};
use hyper::client::Client as HttpClient;
use hyper::Error as HttpError;
use url::{self, ParseError as UrlError, Url};
use oauth2::token::Token;

use auth::{AccessToken, OAuth, Permissions};

pub const VK_DOMAIN: &'static str = "api.vk.com";
pub const VK_PATH: &'static str = "method";

use audio::Audio;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Collection<T: de::Deserialize> {
    pub count: u32,
    pub items: Vec<T>
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct RichCollection<T: de::Deserialize> {
    pub count: u32,
    pub items: Vec<T>,
    pub profiles: Vec<Profile>,
    pub groups: Vec<Group>,
}

impl<T: de::Deserialize + Clone> Clone for Collection<T> {
    fn clone(&self) -> Collection<T> {
        Collection {
            count: self.count,
            items: self.items.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct ApiError {
    pub error_code: ErrorCode,
    pub error_msg: String,
    pub request_params: Vec<KeyVal>
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Copy, Clone)]
pub struct LikesCount {
    pub user_likes: u32,
    pub count: u32,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Profile {
    id: i64,
    first_name: String,
    last_name: String,
}

// TODO: maybe move to groups?
// TODO: must be much more fields
#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Group {
    id: i64,
    name: String,
    screen_name: String,
    is_closed: Bool,
    #[serde(default)]
    is_admin: Bool,
    #[serde(default)]
    is_member: Bool,
    #[serde(rename="type")]
    kind: String,
    photo_50: String, // URL
    photo_100: String, // URL
    photo_200: String, // URL
    status_audio: Option<Audio>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Deserialize)]
pub struct AlbumId {
    pub album_id: Id,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Comment {
    pub id: Id,
    pub from_id: Id,
    pub date: Timestamp,
    pub text: String,
    pub likes: Option<LikesCount>,
}


pub type OwnerId = i64;
pub type Id = u64;
pub type Timestamp = u64;
pub type Duration = u32;
pub type Bool = u8;

#[derive(Copy, Eq, Clone, PartialEq, Debug, Default)]
pub struct FullId(pub OwnerId, pub Id);

impl fmt::Display for FullId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.0, self.1)
    }
}

impl From<(OwnerId, Id)> for FullId {
    fn from(pair: (OwnerId, Id)) -> FullId {
        FullId(pair.0, pair.1)
    }
}

impl Into<(OwnerId, Id)> for FullId {
    fn into(self) -> (OwnerId, Id) {
        (self.0, self.1)
    }
}

pub struct Client {
    client: HttpClient,
}

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Http(HttpError),
    Json(JsonError),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Api(ref err) => err.fmt(f),
            Error::Http(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
        }
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error {
        Error::Api(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::Http(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl From<UrlError> for Error {
    fn from(err: UrlError) -> Error {
        Error::Http(HttpError::Uri(err))
    }
}

pub type Result<T> = StdResult<T, Error>;

impl Client {
    pub fn auth<K, S>(&self, key: K, secret: S) -> OAuth
        where K: Into<String>,
              S: Into<String>
    {
        OAuth::new(&self.client, key.into(), secret.into())
    }

    pub fn new() -> Client {
        Client { client: HttpClient::new() }
    }

    pub fn get<T: Request>(&self, token: Option<&AccessToken>, req: &T) -> Result<T::Response> {
        let url = req.to_url();
        let mut query = req.to_query_string();
        if let Some(ref token) = token {
            query.push_str("&access_token=");
            query.push_str(token.access_token());
        }

        self.client
            .post(url)
            .body(&query)
            .send()
            .map_err(Error::Http)
            .and_then(|resp| serde_json::from_reader::<_, ApiResult<T::Response>>(resp).map_err(Error::Json))
            .and_then(|vkres| vkres.0.map_err(Error::Api))
    }
}

/// Trait for things that can be posted to VK API directly
pub trait Request {
    type Response: de::Deserialize;
    fn method_name() -> &'static str;
    fn to_query_string(&self) -> String;

    fn permissions() -> Permissions {
        Permissions::new(0)
    }

    fn to_url(&self) -> Url {
        Url {
            scheme: String::from("https"),
            scheme_data: url::SchemeData::Relative(url::RelativeSchemeData {
                username: String::new(),
                password: None,
                host: url::Host::Domain(String::from(VK_DOMAIN)),
                port: Some(443),
                default_port: None,
                path: vec![String::from(VK_PATH), String::from(Self::method_name())],
            }),
            query: None,
            fragment: None,
        }
    }

}

#[derive(Debug)]
pub struct ApiResult<T>(pub StdResult<T, ApiError>);

impl<T> Deref for ApiResult<T> {
    type Target = StdResult<T, ApiError>;
    fn deref(&self) -> &StdResult<T, ApiError> {
        &self.0
    }
}

enum ApiResultField {
    Response,
    Error,
}

impl de::Deserialize for ApiResultField {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> StdResult<ApiResultField, D::Error> {
        struct ApiResultFieldVisitor;

        impl de::Visitor for ApiResultFieldVisitor {
            type Value = ApiResultField;
            fn visit_str<E: de::Error>(&mut self, value: &str) -> StdResult<ApiResultField, E> {
                match value {
                    "response" => Ok(ApiResultField::Response),
                    "error" => Ok(ApiResultField::Error),
                    _ => Err(de::Error::unknown_field("expected response or error")),
                }
            }
        }

        d.deserialize(ApiResultFieldVisitor)
    }
}

impl<T: de::Deserialize> de::Deserialize for ApiResult<T> {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> StdResult<ApiResult<T>, D::Error> {
        struct ApiResultVisitor<T: de::Deserialize>(PhantomData<T>);

        impl<T: de::Deserialize> de::Visitor for ApiResultVisitor<T> {
            type Value = ApiResult<T>;
            #[allow(unknown_lints, option_map_unwrap_or_else)]
            fn visit_map<V: de::MapVisitor>(&mut self, mut v: V) -> StdResult<ApiResult<T>, V::Error> {
                v.visit_key()
                 .and_then(|k| {
                     k.map(|k| {
                          match k {
                              ApiResultField::Response => v.visit_value::<T>().map(Ok),
                              ApiResultField::Error => v.visit_value::<ApiError>().map(Err),
                          }
                      })
                      .unwrap_or_else(|| v.missing_field("response or error"))
                 })
                 .and_then(|res| v.end().map(|_| res))
                 .map(ApiResult)
            }
        }

        d.deserialize_map(ApiResultVisitor(PhantomData::<T>))
    }
}

impl Into<(String, String)> for KeyVal {
    fn into(self) -> (String, String) {
        (self.key, self.value)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorCode {
    General, // 1
    Database, // 2
    Unauthorized, // 5
    Signature, // 10
    Request, // 11
    ScriptCompileError, // 12
    ScriptRuntimeError, // 13
    MethodAccessDenied, // 15
    Banned, // 18
    Blocked, // 19
    GoodsNotFound, // 20
    GoodsUnvailable, // 21
    UserNotFound, // 22
    RequiredParameterMissing, // 100
    InvalidAlbumId, // 114
    InvalidServer, // 118
    InvalidHash, // 121
    InvalidPhotoId, // 122
    InvalidAudio, // 123
    InvalidPhoto, // 129
    UserMenuAccessDenied, // 148
    AccessDenied, // 204
    AccessToWallPostDenied, // 210
    PostAddAccessDenied, // 214
    AdsPostWasRecentlyAdded, // 219,
    TooManyRecipients, // 220,
    HyperlinksForbidden, // 222
    UserDisabledTrackBroadcast, // 221
    CopyrightedObjectRemoved, // 270
    InvalidFilename, // 301
    SizeLimitReached, // 302
    VideoAlreadyAdded, // 800
    VideoCommentsClosed, // 801
    App(u32), // 100-999
    Unknown(u32), // other
}

impl From<u32> for ErrorCode {
    fn from(value: u32) -> ErrorCode {
        use self::ErrorCode::*;
        match value {
            1 => General,
            2 => Database,
            5 => Unauthorized,
            10 => Signature,
            11 => Request,
            12 => ScriptCompileError,
            13 => ScriptRuntimeError,
            15 => MethodAccessDenied,
            18 => Banned,
            19 => Blocked,
            20 => GoodsNotFound,
            21 => GoodsUnvailable,
            22 => UserNotFound,
            100 => RequiredParameterMissing,
            114 => InvalidAlbumId,
            118 => InvalidServer,
            121 => InvalidHash,
            122 => InvalidPhotoId,
            123 => InvalidAudio,
            129 => InvalidPhoto,
            148 => UserMenuAccessDenied,
            204 => AccessDenied,
            210 => AccessToWallPostDenied,
            214 => PostAddAccessDenied,
            219 => AdsPostWasRecentlyAdded,
            220 => TooManyRecipients,
            222 => HyperlinksForbidden,
            221 => UserDisabledTrackBroadcast,
            270 => CopyrightedObjectRemoved,
            301 => InvalidFilename,
            302 => SizeLimitReached,
            800 => VideoAlreadyAdded,
            801 => VideoCommentsClosed,
            v @ 100...999 => App(v),
            v => Unknown(v),
        }
    }
}
impl Into<u32> for ErrorCode {
    fn into(self) -> u32 {
        use self::ErrorCode::*;
        match self {
            General => 1,
            Database => 2,
            Unauthorized => 5,
            Signature => 10,
            Request => 11,
            ScriptCompileError => 12,
            ScriptRuntimeError => 13,
            MethodAccessDenied => 15,
            Banned => 18,
            Blocked => 19,
            GoodsNotFound => 20,
            GoodsUnvailable => 21,
            UserNotFound => 22,
            RequiredParameterMissing => 100,
            InvalidAlbumId => 114,
            InvalidServer => 118,
            InvalidHash => 121,
            InvalidPhotoId => 122,
            InvalidAudio => 123,
            InvalidPhoto => 129,
            UserMenuAccessDenied => 148,
            AccessDenied => 204,
            AccessToWallPostDenied => 210,
            PostAddAccessDenied => 214,
            AdsPostWasRecentlyAdded => 219,
            TooManyRecipients => 220,
            HyperlinksForbidden => 222,
            UserDisabledTrackBroadcast => 221,
            CopyrightedObjectRemoved => 270,
            InvalidFilename => 301,
            SizeLimitReached => 302,
            VideoAlreadyAdded => 800,
            VideoCommentsClosed => 801,
            App(v) | Unknown(v) => v,
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorCode::*;
        match *self {
            General => f.write_str("general error"),
            Database => f.write_str("database error"),
            Unauthorized => f.write_str("unauthorized"),
            Signature => f.write_str("invalid signature"),
            Request => f.write_str("invalid request"),
            ScriptCompileError => f.write_str("compile script error"),
            MethodAccessDenied => f.write_str("no access to call this method"),
            ScriptRuntimeError => f.write_str("runtime script error"),
            Banned => f.write_str("banned or deleted"),
            Blocked => f.write_str("content blocked"),
            GoodsNotFound => f.write_str("goods not found"),
            GoodsUnvailable => f.write_str("goods unavailable"),
            UserNotFound => f.write_str("user not found"),
            RequiredParameterMissing => f.write_str("one of required parameters is missing"),
            InvalidAlbumId => f.write_str("invalid album id"),
            InvalidServer => f.write_str("invalid server"),
            InvalidHash => f.write_str("invalid hash"),
            InvalidPhotoId => f.write_str("invalid photo id"),
            InvalidAudio => f.write_str("invalid audio"),
            InvalidPhoto => f.write_str("invalid photo"),
            UserMenuAccessDenied => f.write_str("access to the menu of the user denied"),
            AccessDenied => f.write_str("access denied"),
            AccessToWallPostDenied => f.write_str("access to wall's post denied"),
            PostAddAccessDenied => f.write_str("access to adding post denied"),
            AdsPostWasRecentlyAdded => f.write_str("ads post was recently added"),
            TooManyRecipients => f.write_str("too many recipients"),
            HyperlinksForbidden => f.write_str("hyperlinks are forbidden"),
            UserDisabledTrackBroadcast => f.write_str("user disabled track name broadcast"),
            CopyrightedObjectRemoved => f.write_str("object was removed by copyright holder request"),
            InvalidFilename => f.write_str("invalid filename"),
            SizeLimitReached => f.write_str("object size limit is reached"),
            VideoAlreadyAdded => f.write_str("video is already added"),
            VideoCommentsClosed => f.write_str("comments for this video are closed"),
            App(v) => write!(f, "application error #{}", v),
            Unknown(v) => write!(f, "unknown error #{}", v),
        }
    }
}

impl de::Deserialize for ErrorCode {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> StdResult<ErrorCode, D::Error> {
        u32::deserialize(d).map(From::from)
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        &*self.error_msg
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_code, self.error_msg)
    }
}

pub enum Privacy {
    All,
    Friends,
    FriendsOfFriends,
    FriendsOfFriendsOnly,
    Nobody,
    OnlyMe,
    List(u64),
    OnlyList(u64),
    User(u64),
    OnlyUser(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Sort {
    DateAdded = 0,
    Length = 1,
    Popularity = 2,
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        use self::Sort::*;
        match *self {
            DateAdded => "0",
            Length => "1",
            Popularity => "2",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ReportReason {
    Spam = 0,
    ChildPorn = 1,
    Extremism = 2,
    Violence = 3,
    Drugs = 4,
    AdultOnly = 5,
    Offence = 6,
}

impl Default for ReportReason {
    fn default() -> ReportReason {
        ReportReason::Spam
    }
}

impl AsRef<str> for ReportReason {
    fn as_ref(&self) -> &str {
        use self::ReportReason::*;
        match *self {
            Spam => "0",
            ChildPorn => "1",
            Extremism => "2",
            Violence => "3",
            Drugs => "4",
            AdultOnly => "5",
            Offence => "6",
        }
    }
}

enum_str! { SortOrder {
    Asc = "asc",
    Desc = "desc"
}}


impl Default for SortOrder {
    fn default() -> SortOrder {
        SortOrder::Asc
    }
}

enum_str! { AttachmentKind {
    Photo = "photo",
    Video = "video",
    Audio = "audio",
    Document = "doc",
}}

#[derive(Eq, Copy, Clone, PartialEq, Debug)]
pub struct Attachment {
    pub kind: AttachmentKind,
    pub owner_id: OwnerId,
    pub media_id: Id,
}

impl fmt::Display for Attachment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}_{}", self.kind.as_ref(), self.owner_id, self.media_id)
    }
}
