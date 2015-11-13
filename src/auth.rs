use hyper::client::IntoUrl;
use url::ParseError as UrlError;
use serde::ser::{self, Serialize};
use serde::de::{Deserialize, Deserializer};
use std::borrow::{Cow, Borrow};
use std::ops::Deref;
use std::convert::AsRef;
use std::error::Error;
use std::fmt;

use url::Url;

pub static OAUTH_AUTHORIZE_URL: &'static str = "https://oauth.vk.com/authorize";
pub static OAUTH_ACCESS_TOKEN_URL: &'static str = "https://oauth.vk.com/access_token";
pub static OAUTH_DEFAULT_REDIRECT_URI: &'static str = "https://oauth.vk.com/blank.html";

/// ```rust
/// # extern crate serde_json;
/// # extern crate vkrs;
/// assert_eq!(&*serde_json::to_string(&vkrs::auth::ApiVersion(5, 37)).unwrap(), "\"5.37\"");
/// ```
#[derive(Debug)]
pub struct ApiVersion(pub u32, pub u32);

impl Serialize for ApiVersion {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(&*self.to_string())
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

/// ```rust
/// # extern crate serde_json;
/// # extern crate vkrs;
/// assert_eq!(&*serde_json::to_string(&vkrs::auth::DisplayMode::Page).unwrap(), "\"page\"");
/// ```
#[derive(Debug)]
pub enum DisplayMode {
    Page,
    Popup,
    Mobile,
}

impl AsRef<str> for DisplayMode {
    fn as_ref(&self) -> &str {
        use self::DisplayMode::*;
        match *self {
            Page => "page",
            Popup => "popup",
            Mobile => "mobile",
        }
    }
}

impl Serialize for DisplayMode {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(self.as_ref())
    }
}

#[derive(Debug)]
pub enum ResponseMode {
    Code,
    Token,
}

impl AsRef<str> for ResponseMode {
    fn as_ref(&self) -> &str {
        use self::ResponseMode::*;
        match *self {
            Code => "code",
            Token => "token",
        }
    }
}

impl Serialize for ResponseMode {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(self.as_ref())
    }
}

#[derive(Debug)]
pub enum Permission {
    Notify,
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
    Ads,
    Offline,
    NoHttps,
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

impl Serialize for Permission {
    fn serialize<S: ser::Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(self.as_ref())
    }
}

/// ```rust
/// # use vkrs::auth::{OAuthReq, Permission};
/// # use hyper::client::IntoUrl;
/// let mut auth_req = OAuthReq::new(env!("VK_APP_ID"));
/// auth_req.scope(Permission::Audio);
/// assert_eq!(auth_req.to_url().serialize(),
///     "https://oauth.vk.com/authorize?client_id=5093489&redirect_uri=https%3A%2F%2Foauth.vk.com%2Fblank.html&display=page&scope=audio&response_type=code&v=5.37");
/// ```
#[derive(Debug, Serialize)]
pub struct OAuthReq<'a> {
    client_id: Cow<'a, str>,
    redirect_uri: Option<Cow<'a, str>>,
    display: DisplayMode,
    scope: Vec<Permission>,
    response_type: ResponseMode,
    v: ApiVersion,
    state: Option<Cow<'a, str>>,
}

impl<'a> OAuthReq<'a> {
    pub fn new<'b: 'a, T: Into<Cow<'b, str>>>(client_id: T) -> OAuthReq<'a> {
        OAuthReq {
            client_id: client_id.into(),
            redirect_uri: None,
            display: DisplayMode::Page,
            scope: vec![],
            response_type: ResponseMode::Code,
            v: ApiVersion(5, 37),
            state: None
        }
    }

    pub fn redirect_uri<'b: 'a, T: Into<Cow<'b, str>>>(&mut self, redirect_uri: T) -> &mut Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    pub fn scopes<I: IntoIterator<Item=Permission>>(&mut self, scopes: I) -> &mut Self {
        self.scope.extend(scopes);
        self
    }

    pub fn scope(&mut self, scope: Permission) -> &mut Self {
        self.scope.push(scope);
        self
    }

    pub fn display(&mut self, display: DisplayMode) -> &mut Self {
        self.display = display;
        self
    }

    pub fn to_access_token_req<T: Into<Cow<'a, str>>, U: Into<Cow<'a, str>>>(&self, client_secret: T, code: U) -> AccessTokenReq<'a> {
        let mut access_token_req = AccessTokenReq::new(self.client_id.clone(), client_secret, code);
        access_token_req.redirect_uri = self.redirect_uri.clone();
        access_token_req
    }
}

impl<'a> IntoUrl for &'a OAuthReq<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut url = try!(OAUTH_AUTHORIZE_URL.into_url());
        let redir_url = if let Some(ref url) = self.redirect_uri {
            url.borrow()
        } else {
            OAUTH_DEFAULT_REDIRECT_URI
        };

        url.set_query_from_pairs([
            ("client_id", self.client_id.borrow()),
            ("redirect_uri", redir_url),
            ("display", self.display.as_ref()),
            ("scope", &*self.scope.iter().map(Permission::as_ref).collect::<Vec<_>>().join(",")),
            ("response_type", self.response_type.as_ref()),
            ("v", &*self.v.to_string()),
        ].iter().cloned());

        Ok(url)
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenReq<'a> {
    client_id: Cow<'a, str>,
    client_secret: Cow<'a, str>,
    redirect_uri: Option<Cow<'a, str>>,
    code: Cow<'a, str>,
}

impl<'a> AccessTokenReq<'a> {
    pub fn new<T, U, V>(client_id: T, client_secret: U, code: V) -> AccessTokenReq<'a>
        where T: Into<Cow<'a, str>>, U: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>
    {
        AccessTokenReq {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: None,
            code: code.into(),
        }
    }
}

impl<'a> IntoUrl for &'a AccessTokenReq<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut url = try!(OAUTH_ACCESS_TOKEN_URL.into_url());
        let redir_url = if let Some(ref url) = self.redirect_uri {
            url.borrow()
        } else {
            OAUTH_DEFAULT_REDIRECT_URI
        };

        url.set_query_from_pairs([
            ("client_id", self.client_id.borrow()),
            ("client_secret", self.client_secret.borrow()),
            ("redirect_uri", redir_url),
            ("code", self.code.borrow()),
        ].iter().cloned());

        Ok(url)
    }
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResp {
    access_token: String,
    expires_in: u32,
    user_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResp {
    error: String,
    error_description: String,
}

impl Error for ErrorResp {
    fn description(&self) -> &str {
        &*self.error_description
    }
}

impl fmt::Display for ErrorResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.error)
    }
}

impl Deref for AccessTokenResp {
    type Target = str;
    fn deref(&self) -> &str {
        &*self.access_token
    }
}

impl<'a> Into<Cow<'a, str>> for &'a AccessTokenResp {
    fn into(self) -> Cow<'a, str> {
        Cow::Borrowed(&*self)
    }
}

pub trait WithToken<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&'a mut self, token: T) -> &'a mut Self;
}

#[derive(Debug)]
pub struct AccessTokenResult(pub Result<AccessTokenResp, ErrorResp>);

impl Deserialize for AccessTokenResult {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<AccessTokenResult, D::Error> {
        AccessTokenResp::deserialize(d).map(|v| AccessTokenResult(Ok(v)))
            .or_else(|_| ErrorResp::deserialize(d).map(|e| AccessTokenResult(Err(e))))
    }
}

impl Deref for AccessTokenResult {
    type Target = Result<AccessTokenResp, ErrorResp>;
    fn deref(&self) -> &Result<AccessTokenResp, ErrorResp> {
        &self.0
    }
}
