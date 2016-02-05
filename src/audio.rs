use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use serde::de;
use super::api::{Bool, Collection, Date, Duration, FullId, Id, OwnerId, Sort};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genre {
    Rock, // 1
    Pop, // 2
    RapHipHop, // 3
    EasyListen, // 4
    DanceHouse, // 5
    Instrumental, // 6
    Metal, // 7
    Alternative, // 21
    Dubstep, // 8
    JazzBlues, // 9
    DrumBass, // 10
    Trance, // 11
    Chanson, // 12
    Ethnic, // 13
    AcousticVocal, // 14
    Reggae, // 15
    Classical, // 16
    IndiePop, // 17
    Speech, // 19
    ElectropopDisco, // 22
    Other, // 18
    Unknown(u32),
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Genre::*;
        match *self {
            Rock => f.write_str("rock"),
            Pop => f.write_str("pop"),
            RapHipHop => f.write_str("rap & hiphop"),
            EasyListen => f.write_str("easy listening"),
            DanceHouse => f.write_str("dance & house"),
            Instrumental => f.write_str("dance & house"),
            Metal => f.write_str("metal"),
            Alternative => f.write_str("alternative"),
            Dubstep => f.write_str("dubstep"),
            JazzBlues => f.write_str("jazz & blues"),
            DrumBass => f.write_str("drum & bass"),
            Trance => f.write_str("trance"),
            Chanson => f.write_str("chanson"),
            Ethnic => f.write_str("ethnic"),
            AcousticVocal => f.write_str("acoustic & vocal"),
            Reggae => f.write_str("reggae"),
            Classical => f.write_str("classical"),
            IndiePop => f.write_str("indie pop"),
            Speech => f.write_str("speech"),
            ElectropopDisco => f.write_str("electro pop & disco"),
            Other => f.write_str("other"),
            Unknown(id) => write!(f, "unknown (#{})", id),
        }
    }
}

impl de::Deserialize for Genre {
    fn deserialize<D: de::Deserializer>(d: &mut D) -> Result<Genre, D::Error> {
        use self::Genre::*;
        de::Deserialize::deserialize(d).and_then(|v: u32| {
            match v {
                1 => Ok(Rock),
                2 => Ok(Pop),
                3 => Ok(RapHipHop),
                4 => Ok(EasyListen),
                5 => Ok(DanceHouse),
                6 => Ok(Instrumental),
                7 => Ok(Metal),
                21 => Ok(Alternative),
                8 => Ok(Dubstep),
                9 => Ok(JazzBlues),
                10 => Ok(DrumBass),
                11 => Ok(Trance),
                12 => Ok(Chanson),
                13 => Ok(Ethnic),
                14 => Ok(AcousticVocal),
                15 => Ok(Reggae),
                16 => Ok(Classical),
                17 => Ok(IndiePop),
                19 => Ok(Speech),
                22 => Ok(ElectropopDisco),
                18 => Ok(Other),
                v => Ok(Unknown(v)),
            }
        })
    }
}

impl Into<u32> for Genre {
    fn into(self) -> u32 {
        use self::Genre::*;
        match self {
            Rock => 1,
            Pop => 2,
            RapHipHop => 3,
            EasyListen => 4,
            DanceHouse => 5,
            Instrumental => 6,
            Metal => 7,
            Alternative => 21,
            Dubstep => 8,
            JazzBlues => 9,
            DrumBass => 10,
            Trance => 11,
            Chanson => 12,
            Ethnic => 13,
            AcousticVocal => 14,
            Reggae => 15,
            Classical => 16,
            IndiePop => 17,
            Speech => 19,
            ElectropopDisco => 22,
            Other => 18,
            Unknown(v) => v,
        }
    }
}

request! {
    #[derive(Eq)]
    struct Get for ["audio.get"](v => 5.37, need_user => 0) -> Collection<Audio> {
        owner_id: OwnerId = () => {},
        album_id: Option<Id> = () => { |value| value.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or("") },
        audio_ids: Vec<Id> = () => { Vec },
        offset: usize = (0) => {},
        count: usize = (100) => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["audio.search"](v => 5.44) -> Collection<Audio> {
        sized {
            auto_complete: bool = () => {bool},
            lyrics: bool = () => {bool},
            performer_only: bool = () => {bool},
            sort: Sort = (Sort::Popularity) => {AsRef},
            search_own: bool = () => {bool},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            q: str = ("") => {=},
        }
    }
}

#[cfg(feature = "unstable")]
include!("audio.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/audio.rs"));

request_ref! {
    #[derive(Copy, Eq)]
    struct GetById for ["audio.getById"](v => 5.44) -> Collection<Audio> {
        audios: [FullId] = (&[][..]) => {|value|
            &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
        }
    }
}

request! {
    #[derive(Copy, Eq)]
    struct GetLyrics for ["audio.getLyrics"](v => 5.44) -> Lyrics {
        lyrics_id: Id = () => {}
    }
}

request! {
    #[derive(Copy, Eq)]
    struct GetCount for ["audio.getCount"](v => 5.44) -> u64 {
        owner_id: OwnerId = () => {}
    }
}

request! {
    #[derive(Copy, Eq)]
    struct GetAlbums for ["audio.getAlbums"](v => 5.44) -> Collection<Album> {
        owner_id: OwnerId = () => {},
        offset: usize = (0) => {},
        count: usize = (30) => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetPopular for ["audio.getPopular"](v => 5.44) -> Vec<Audio> {
        only_eng: bool = () => {bool},
        genre_id: Option<Genre> = (None) => {
            |value| value.map(Into::<u32>::into).as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or("")
        },
        offset: usize = (0) => {},
        count: usize = (30) => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetRecommendations for ["audio.getRecommendations"](v => 5.44) -> Collection<Audio> {
        target_audio: Option<FullId> = (None) => { |value|
            value.map(|(x, y)| format!("{}_{}", x, y)).as_ref().map(Borrow::borrow).unwrap_or("")
        },
        user_id: Option<Id> = () => {Option},
        offset: usize = (0) => {},
        count: usize = (30) => {},
        shuffle: bool = () => {bool},
    }
}
