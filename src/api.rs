use std::fmt;
use std::borrow::Cow;
use std::ops::Deref;
use std::marker::PhantomData;
use std::error::Error;
use serde::de;
use hyper::client::IntoUrl;

pub const VK_METHOD_URL: &'static str = "https://api.vk.com/method/";

pub trait WithToken<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&'a mut self, token: T) -> &'a mut Self;
}

pub trait Request<'a> where &'a Self: IntoUrl, Self: 'a {
    const METHOD_NAME: &'static str;
}

#[derive(Debug)]
pub struct VkResult<T: fmt::Debug>(pub Result<T, VkError>);

impl<T: fmt::Debug> Deref for VkResult<T> {
    type Target = Result<T, VkError>;
    fn deref(&self) -> &Result<T, VkError> {
        &self.0
    }
}

enum VkResultField {
    Response,
    Error
}

impl de::Deserialize for VkResultField {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<VkResultField, D::Error> {
        struct VkResultFieldVisitor;

        impl de::Visitor for VkResultFieldVisitor {
            type Value = VkResultField;
            fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<VkResultField, E> {
                match value {
                    "response" => Ok(VkResultField::Response),
                    "error" => Ok(VkResultField::Error),
                    _ => Err(de::Error::syntax("expected response or error"))
                }
            }
        }

        d.visit(VkResultFieldVisitor)
    }
}

impl<T: de::Deserialize + fmt::Debug> de::Deserialize for VkResult<T> {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<VkResult<T>, D::Error> {
        struct VkResultVisitor<T: de::Deserialize + fmt::Debug>(PhantomData<T>);

        impl<T: de::Deserialize + fmt::Debug> de::Visitor for VkResultVisitor<T> {
            type Value = VkResult<T>;
            fn visit_map<V: de::MapVisitor>(&mut self, mut v: V) -> Result<VkResult<T>, V::Error> {
                match v.visit_key() {
                    Ok(Some(VkResultField::Response)) => v.visit_value::<T>().map(|v| VkResult(Ok(v))),
                    Ok(Some(VkResultField::Error)) => v.visit_value::<VkError>().map(|e| VkResult(Err(e))),
                    Ok(None) => v.missing_field("response or error"),
                    Err(err) => Err(err)
                }
            }
        }

        d.visit_map(VkResultVisitor(PhantomData::<T>))
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyVal {
    pub key: String,
    pub value: String
}

impl Into<(String, String)> for KeyVal {
    fn into(self) -> (String, String) {
        (self.key, self.value)
    }
}

#[derive(Debug, Deserialize)]
pub struct VkError {
    pub error_code: VkErrorCode,
    pub error_msg: String,
    pub request_params: Vec<KeyVal>
}

#[derive(Debug)]
pub enum VkErrorCode {
    General, // 1
    Database, // 2
    Unauthorized, // 5
    Signature, // 10
    Request, // 11
    GoodsNotFound, // 20
    GoodsUnvailable, // 21
    UserNotFound, // 22
    App(u32), // 100-999
    Unknown(u32), // other
}

impl From<u32> for VkErrorCode {
    fn from(value: u32) -> VkErrorCode {
        use self::VkErrorCode::*;
        match value {
            1 => General,
            2 => Database,
            5 => Unauthorized,
            10 => Signature,
            11 => Request,
            20 => GoodsNotFound,
            21 => GoodsUnvailable,
            22 => UserNotFound,
            v @ 100...999 => App(v),
            v @ _ => Unknown(v)
        }
    }
}
impl Into<u32> for VkErrorCode {
    fn into(self) -> u32 {
        use self::VkErrorCode::*;
        match self {
            General => 1,
            Database => 2,
            Unauthorized => 5,
            Signature => 10,
            Request => 11,
            GoodsNotFound => 20,
            GoodsUnvailable => 21,
            UserNotFound => 22,
            App(v) => v,
            Unknown(v) => v,
        }
    }
}

impl fmt::Display for VkErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::VkErrorCode::*;
        match *self {
            General => f.write_str("general error"),
            Database => f.write_str("database error"),
            Unauthorized => f.write_str("access denied"),
            Signature => f.write_str("invalid signature"),
            Request => f.write_str("invalid request"),
            GoodsNotFound => f.write_str("goods not found"),
            GoodsUnvailable => f.write_str("goods unavailable"),
            UserNotFound => f.write_str("user not found"),
            App(v) => write!(f, "application error #{}", v),
            Unknown(v) => write!(f, "unknown error #{}", v),
        }
    }
}

impl de::Deserialize for VkErrorCode {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<VkErrorCode, D::Error> {
        u32::deserialize(d).map(From::from)
    }
}

impl Error for VkError {
    fn description(&self) -> &str {
        &*self.error_msg
    }
}

impl fmt::Display for VkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_code, self.error_msg)
    }
}

