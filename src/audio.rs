pub struct User {
    id: i64, // String
    photo: String,
    name: String,
    name_gen: String,
}

pub struct Audio {
    id: u64,
    owner_id: i64,
    artist: String,
    title: String,
    duration: u32,
    date: u64,
    url: String, // Url !!!
    lyrics_id: Option<u64>,
    genre_id: u32,
}

pub struct AudioGetReq<'a> {
     owner_id: i64,
     album_id: u64,
     audio_ids: Cow<'a, [u64]>,
     need_user: bool,
     offset: usize,
     count: usize,
}
pub enum AudioSort {
    DateAdded = 0,
    Length = 1,
    Popularity = 2,
}
pub struct AudioSearchReq<'a> {
     q: Cow<'a, str>,
     auto_complete: bool,
     lyrics: bool,
     performer_only: bool,
     sort: AudioSort,
     search_own: bool,
     offset: usize,
     count: usize, // 0...300, def 30
}

pub struct AudioGetResp {
    count: u32,
    items: Vec<Audio>
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

