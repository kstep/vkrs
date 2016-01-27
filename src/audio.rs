use std::borrow::{Cow, Borrow};
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use std::ops::Deref;
use hyper::Url;
use hyper::client::IntoUrl;
use url::{ParseError as UrlError};
use super::api::{Request, Response, Collection};

#[derive(Debug)]
pub struct Get {
     owner_id: i64,
     album_id: Option<u64>,
     audio_ids: Option<Vec<u64>>,
     need_user: bool,
     offset: usize,
     count: usize,
}

impl Get {
    pub fn new(owner_id: i64) -> Get {
        Get {
            owner_id: owner_id,
            album_id: None,
            audio_ids: None,
            need_user: false,
            offset: 0,
            count: 100,
        }
    }

    pub fn audios<I: Iterator<Item=u64>>(&mut self, audio_ids: I) -> &mut Get {
        self.audio_ids = Some(audio_ids.collect::<Vec<_>>());
        self
    }

    pub fn album(&mut self, album_id: u64) -> &mut Get {
        self.album_id = Some(album_id);
        self
    }

    pub fn count(&mut self, count: usize) -> &mut Get {
        self.count = count;
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Get {
        self.offset = offset;
        self
    }
}

impl<'a> Request<'a> for Get {
    type Response = Collection<Audio>;
    fn method_name() -> &'static str { "audio.get" }
}

impl<'a> IntoUrl for &'a Get {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(Get::base_url(qs![
            owner_id => &*self.owner_id.to_string(),
            album_id => self.album_id.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or(""),
            audio_ids => &*self.audio_ids.as_ref().map(Deref::deref).unwrap_or(&[]).iter()
                .map(ToString::to_string).collect::<Vec<_>>().join(","),
            need_user => "0",
            offset => &*self.offset.to_string(),
            v => "5.37",
        ]))
    }
}

#[derive(Debug)]
pub struct Search<'a> {
     q: Cow<'a, str>,
     auto_complete: bool,
     lyrics: bool,
     performer_only: bool,
     sort: Sort,
     search_own: bool,
     offset: usize,
     count: usize, // 0...300, def 30
}

impl<'a> Search<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(query: T) -> Search<'a> {
        Search {
            q: query.into(),
            auto_complete: false,
            lyrics: false,
            performer_only: false,
            sort: Sort::Popularity,
            search_own: false,
            offset: 0,
            count: 30,
        }
    }

    pub fn performer_only(&mut self, performer_only: bool) -> &mut Search<'a> {
        self.performer_only = performer_only;
        self
    }
    pub fn search_own(&mut self, search_own: bool) -> &mut Search<'a> {
        self.search_own = search_own;
        self
    }
    pub fn lyrics(&mut self, lyrics: bool) -> &mut Search<'a> {
        self.lyrics = lyrics;
        self
    }

    pub fn count(&mut self, count: usize) -> &mut Search<'a> {
        self.count = count;
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Search<'a> {
        self.offset = offset;
        self
    }

    pub fn sort(&mut self, sort: Sort) -> &mut Search<'a> {
        self.sort = sort;
        self
    }
}

impl<'a> Request<'a> for Search<'a> {
    type Response = Collection<Audio>;
    fn method_name() -> &'static str { "audio.search" }
}

impl<'a> IntoUrl for &'a Search<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(Search::base_url(qs![
            q => self.q.borrow(),
            auto_complete => if self.auto_complete {"1"} else {"0"},
            lyrics => if self.lyrics {"1"} else {"0"},
            performer_only => if self.performer_only {"1"} else {"0"},
            sort => self.sort.as_ref(),
            search_own => if self.search_own {"1"} else {"0"},
            offset => &*self.offset.to_string(),
            count => &*self.count.to_string(),
            v => "5.37",
        ]))
    }
}

#[derive(Debug)]
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

#[cfg(feature = "nightly")]
include!("audio.rs.in");

#[cfg(not(feature = "nightly"))]
include!(concat!(env!("OUT_DIR"), "/audio.rs"));

impl Response for Audio {}

pub struct GetById<'a> {
    pub audios: &'a [(i64, u64)]
}

impl<'a> Request<'a> for GetById<'a> {
    type Response = Collection<Audio>;
    fn method_name() -> &'static str { "audio.getById" }
}

impl<'a> IntoUrl for &'a GetById<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(GetById::base_url(qs![
            audios => &*self.audios.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(","),
            v => "5.44",
        ]))
    }
}

pub struct GetLyrics {
    lyrics_id: u64
}

impl GetLyrics {
    pub fn new(id: u64) -> GetLyrics {
        GetLyrics {
            lyrics_id: id
        }
    }
}

impl Response for Lyrics {}

impl<'a> Request<'a> for GetLyrics {
    type Response = Lyrics;
    fn method_name() -> &'static str { "audio.getLyrics" }
}

impl<'a> IntoUrl for &'a GetLyrics {
    fn into_url(self) -> Result<Url, UrlError> {
        Ok(GetLyrics::base_url(qs![
            lyrics_id => &*self.lyrics_id.to_string(),
            v => "5.44",
        ]))
    }
}

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

