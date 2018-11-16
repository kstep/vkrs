use std::borrow::Borrow;
use api::Collection;
use serde::de;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct GiftItem {
    pub id: u64,
    pub from_id: i64,
    pub message: String,
    pub date: u64,
    pub gift: Gift,
    pub privacy: GiftPrivacy,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Gift {
    pub id: u64,
    pub thumb_256: String, // Url
    pub thumb_96: String, // Url
    pub thumb_48: String, // Url
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GiftPrivacy {
    Pubclic, // 0
    SenderNameOnly, // 1
    Private, // 2
}

impl de::Deserialize for GiftPrivacy {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<GiftPrivacy, D::Error> {
        use self::GiftPrivacy::*;
        de::Deserialize::deserialize(d).and_then(|v: u32| {
            match v {
                0 => Ok(Pubclic),
                1 => Ok(SenderNameOnly),
                2 => Ok(Private),
                _ => unreachable!(),
            }
        })
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Get for ["gifts.get"](v => 5.44) -> Collection<GiftItem> {
        user_id: Option<i64> = () => {Option},
        count: Option<u64> = () => {Option},
        offset: Option<u64> = () => {Option},
    }
}
