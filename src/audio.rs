use std::borrow::{Cow, Borrow};
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use hyper::Url;
use hyper::client::IntoUrl;
use url::ParseError as UrlError;
use super::api::{WithToken, Request, Response, VK_METHOD_URL};

#[derive(Debug)]
pub struct Get<'a> {
     owner_id: i64,
     album_id: u64,
     audio_ids: Cow<'a, [u64]>,
     need_user: bool,
     offset: usize,
     count: usize,
     token: Option<Cow<'a, str>>,
}

impl<'a> WithToken<'a> for Get<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&'a mut self, token: T) -> &'a mut Get<'a> {
        self.token = Some(token.into());
        self
    }
}

impl<'a> Request<'a> for Get<'a> {
    const METHOD_NAME: &'static str = "audio.get";
}

impl<'a> IntoUrl for &'a Get<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut url = try!(Url::parse(&*(VK_METHOD_URL.to_owned() + Get::METHOD_NAME)));
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
pub struct Search<'a> {
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

impl<'a> WithToken<'a> for Search<'a> {
    fn with_token<T: Into<Cow<'a, str>>>(&mut self, token: T) -> &mut Search<'a> {
        self.token = Some(token.into());
        self
    }
}

impl<'a> Search<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(query: T) -> Search<'a> {
        Search {
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

    pub fn sort(&mut self, sort: AudioSort) -> &mut Search<'a> {
        self.sort = sort;
        self
    }
}

impl<'a> Request<'a> for Search<'a> {
    const METHOD_NAME: &'static str = "audio.search";
}

impl<'a> IntoUrl for &'a Search<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut url = Url::parse(&*(VK_METHOD_URL.to_owned() + Search::METHOD_NAME)).unwrap();
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

impl Response for Audio {}

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

