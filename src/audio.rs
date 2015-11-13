use std::borrow::{Cow, Borrow};
use std::convert::AsRef;
use std::string::ToString;
use std::marker::PhantomData;
use std::error::Error;
use std::ops::Deref;
use hyper::Url;
use hyper::client::IntoUrl;
use url::ParseError as UrlError;
use serde::de::{self, Deserialize, Deserializer};
use std::fmt::{self, Debug};
use super::auth::WithToken;

const VK_METHOD_URL: &'static str = "https://api.vk.com/method/";

#[derive(Debug)]
pub struct AudioGetReq<'a> {
     owner_id: i64,
     album_id: u64,
     audio_ids: Cow<'a, [u64]>,
     need_user: bool,
     offset: usize,
     count: usize,
     token: Option<Cow<'a, str>>,
}

impl<'a> WithToken<'a> for AudioGetReq<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&'a mut self, token: T) -> &'a mut AudioGetReq<'a> {
        self.token = Some(token.into());
        self
    }
}

impl<'a> IntoUrl for &'a AudioGetReq<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut url = try!(Url::parse(&*(VK_METHOD_URL.to_owned() + "audio.get")));
        let audio_ids: &[u64] = self.audio_ids.borrow();
        url.set_query_from_pairs([
                                 ("owner_id", &*self.owner_id.to_string()),
                                 ("album_id", &*self.album_id.to_string()),
                                 ("audio_ids", &*audio_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",")),
                                 ("need_user", "0"),
                                 ("offset", &*self.offset.to_string()),
                                 ("count", &*self.count.to_string()),
                                 ("v", "5.37"),
                                 ("access_token", self.token.as_ref().unwrap().borrow()),
                                 ].iter().cloned());
        Ok(url)
    }
}

#[derive(Debug)]
pub struct AudioSearchReq<'a> {
     q: Cow<'a, str>,
     auto_complete: bool,
     lyrics: bool,
     performer_only: bool,
     sort: AudioSort,
     search_own: bool,
     offset: usize,
     count: usize, // 0...300, def 30
     token: Option<Cow<'a, str>>,
}

impl<'a> WithToken<'a> for AudioSearchReq<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&mut self, token: T) -> &mut AudioSearchReq<'a> {
        self.token = Some(token.into());
        self
    }
}

impl<'a> AudioSearchReq<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(query: T) -> AudioSearchReq<'a> {
        AudioSearchReq {
            q: query.into(),
            auto_complete: false,
            lyrics: false,
            performer_only: false,
            sort: AudioSort::Popularity,
            search_own: false,
            offset: 0,
            count: 30,
            token: None,
        }
    }

    pub fn performer_only(&mut self, performer_only: bool) -> &mut AudioSearchReq<'a> {
        self.performer_only = performer_only;
        self
    }
    pub fn search_own(&mut self, search_own: bool) -> &mut AudioSearchReq<'a> {
        self.search_own = search_own;
        self
    }
    pub fn lyrics(&mut self, lyrics: bool) -> &mut AudioSearchReq<'a> {
        self.lyrics = lyrics;
        self
    }

    pub fn count(&mut self, count: usize) -> &mut AudioSearchReq<'a> {
        self.count = count;
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut AudioSearchReq<'a> {
        self.offset = offset;
        self
    }

    pub fn sort(&mut self, sort: AudioSort) -> &mut AudioSearchReq<'a> {
        self.sort = sort;
        self
    }
}

impl<'a> IntoUrl for &'a AudioSearchReq<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut url = Url::parse(&*(VK_METHOD_URL.to_owned() + "audio.search")).unwrap();
        url.set_query_from_pairs([
                                 ("q", self.q.borrow()),
                                 ("auto_complete", if self.auto_complete {"1"} else {"0"}),
                                 ("lyrics", if self.lyrics {"1"} else {"0"}),
                                 ("performer_only", if self.performer_only {"1"} else {"0"}),
                                 ("sort", self.sort.as_ref()),
                                 ("search_own", if self.search_own {"1"} else {"0"}),
                                 ("offset", &*self.offset.to_string()),
                                 ("count", &*self.count.to_string()),
                                 ("v", "5.37"),
                                 ("access_token", self.token.as_ref().unwrap().borrow())
                                 ].iter().cloned());
        Ok(url)
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum AudioSort {
    DateAdded = 0,
    Length = 1,
    Popularity = 2,
}

impl AsRef<str> for AudioSort {
    fn as_ref(&self) -> &str {
        use self::AudioSort::*;
        match *self {
            DateAdded => "0",
            Length => "1",
            Popularity => "2",
        }
    }
}

#[derive(Debug)]
pub struct VkResult<T: Debug>(pub Result<T, VkError>);

impl<T: Debug> Deref for VkResult<T> {
    type Target = Result<T, VkError>;
    fn deref(&self) -> &Result<T, VkError> {
        &self.0
    }
}

enum VkResultField {
    Response,
    Error
}

impl Deserialize for VkResultField {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<VkResultField, D::Error> {
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

impl<T: Deserialize + Debug> Deserialize for VkResult<T> {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<VkResult<T>, D::Error> {
        struct VkResultVisitor<T: Deserialize + Debug>(PhantomData<T>);

        impl<T: Deserialize + Debug> de::Visitor for VkResultVisitor<T> {
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
    pub error_code: u32,
    pub error_msg: String,
    pub request_params: Vec<KeyVal>
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

#[derive(Debug, Deserialize)]
pub struct AudioGetResp {
    pub count: u32,
    pub items: Vec<Audio>
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64, // String
    pub photo: String,
    pub name: String,
    pub name_gen: String,
}

#[derive(Debug, Deserialize)]
pub struct Audio {
    pub id: u64,
    pub owner_id: i64,
    pub artist: String,
    pub title: String,
    pub date: u64,
    pub url: String, // Url !!!
    pub lyrics_id: Option<u64>,
    pub album_id: Option<u64>,
    pub genre_id: Option<u32>,
    pub duration: u32,
}

// audio.get Возвращает список аудиозаписей пользователя или сообщества.
//     owner_id: i64,
//     album_id: u64,
//     audio_ids: &'a [u64],
//     need_user: bool,
//     offset: usize,
//     count: usize,
//
// audio.getById Возвращает информацию об аудиозаписях.
//     audios: &'a [(i64, u64)]
//
// audio.getLyrics Возвращает текст аудиозаписи.
//     lyrics_id: u64
//
// audio.search Возвращает список аудиозаписей в соответствии с заданным критерием поиска.
//     q: Cow<'a, str>,
//     auto_complete: bool,
//     lyrics: bool,
//     performer_only: bool,
//     sort: enum AudioSort { DateAdded = 0, Length = 1, Popularity = 2 }
//     search_own: bool,
//     offset: usize,
//     count: usize, // 0...300, def 30
//
//     Resp:
//     count: u64,
//     items: Vec<Audio {
//         id: u64,
//         owner_id: i64,
//         artist: String,
//         title: String,
//         duration: u32,
//         date: u64,
//         url: String, // !!!
//         lyrics_id: u64,
//         genre_id: u32,
//     }>
//
//     User {
//         id: i64, // String
//         photo: String,
//         name: String,
//         name_gen: String,
//     }
//
// audio.getUploadServer Возвращает адрес сервера для загрузки аудиозаписей.
// audio.save Сохраняет аудиозаписи после успешной загрузки.
// audio.add Копирует аудиозапись на страницу пользователя или группы.
// audio.delete Удаляет аудиозапись со страницы пользователя или сообщества.
// audio.edit Редактирует данные аудиозаписи на странице пользователя или сообщества.
// audio.reorder Изменяет порядок аудиозаписи, перенося ее между аудиозаписями, идентификаторы которых переданы параметрами after и before.
// audio.restore Восстанавливает аудиозапись после удаления.
// audio.getAlbums Возвращает список альбомов аудиозаписей пользователя или группы.
// audio.addAlbum Создает пустой альбом аудиозаписей.
// audio.editAlbum Редактирует название альбома аудиозаписей.
// audio.deleteAlbum Удаляет альбом аудиозаписей.
// audio.moveToAlbum Перемещает аудиозаписи в альбом.
// audio.setBroadcast Транслирует аудиозапись в статус пользователю или сообществу.
// audio.getBroadcast ListВозвращает список друзей и сообществ пользователя, которые транслируют музыку в статус.
// audio.getRecommendations Возвращает список рекомендуемых аудиозаписей на основе списка воспроизведения заданного пользователя или на основе одной выбранной аудиозаписи.
// audio.getPopular Возвращает список аудиозаписей из раздела "Популярное".
// audio.getCount Возвращает количество аудиозаписей пользователя или сообщества.

