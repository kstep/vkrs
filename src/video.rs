use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use std::fmt;
use api::{Bool, Collection, Duration, FullId, Id, LikesCount, OwnerId, Sort, Timestamp, ReportReason};
use serde::de::Deserialize;

#[cfg(feature = "unstable")]
include!("video.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

request_ref! {
    #[derive(Eq, Copy)]
    struct Get for ["video.get"](v => 5.44) -> Collection<Video> [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            videos: [FullId] = (&[][..]) => { |value|
                &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
            }
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["video.search"](v => 5.44) -> Collection<Video> [Video] {
        sized {
            sort: Sort = (Sort::Popularity) => {AsRef},
            hd: bool = () => {bool},
            adult: bool = () => {bool},
            search_own: bool = () => {bool},
            longer: Option<usize> = () => {Option},
            shorter: Option<usize> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            filters: [Filter] = (&[][..]) => {AsRef<Vec>},
            q: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Edit for ["video.edit"](v => 5.44) -> Bool [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            video_id: Id = () => {},
            no_comments: bool = () => {bool},
            repeat: bool = () => {bool},
        }
        unsized {
            name: str = ("") => {=},
            desc: str = ("") => {=},
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Add for ["video.add"](v => 5.44) -> Bool [Video] {
        target_id: OwnerId = () => {},
        video_id: Id = () => {},
        owner_id: OwnerId = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Save for ["video.save"](v => 5.44) -> SavedVideo [Video] {
        sized {
            is_private: bool = (true) => {bool},
            wallpost: bool = () => {bool},
            group_id: Id = () => {},
            album_id: Id = () => {},
            no_comments: bool = () => {bool},
            repeat: bool = () => {bool},
        }
        unsized {
            name: str = ("") => {=},
            description: str = ("") => {=},
            link: str = ("") => {=},
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Delete for ["video.delete"](v => 5.44) -> Bool [Video] {
        video_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
        target_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Restore for ["video.restore"](v => 5.44) -> Bool [Video] {
        video_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetUserVideos for ["video.getUserVideos"](v => 5.44, extended => 0) -> Collection<Video> [Video] {
        user_id: Option<Id> = () => {Option},
        offset: usize = (0) => {},
        count: usize = (30) => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetAlbums for ["video.getAlbums"](v => 5.44) -> Collection<Album> [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        offset: usize = (0) => {},
        count: usize = (30) => {},
        need_system: bool = () => {bool},
        extended: bool = (true) => {bool},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetAlbumById for ["video.getAlbumById"](v => 5.44) -> Album [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        album_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct AddAlbum for ["video.addAlbum"](v => 5.44) -> AlbumId [Video] {
        sized {
            group_id: Option<Id> = () => {Option},
            privacy: Privacy = (Privacy::OnlyMe) => {AsRef},
        }
        unsized {
            title: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct EditAlbum for ["video.editAlbum"](v => 5.44) -> Bool [Video] {
        sized {
            group_id: Option<Id> = () => {Option},
            album_id: Id = () => {},
            privacy: Option<Privacy> = () => {AsRef<Option>},
        }
        unsized {
            title: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct DeleteAlbum for ["video.deleteAlbum"](v => 5.44) -> Bool [Video] {
        group_id: Option<Id> = () => {Option},
        album_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct ReorderAlbums for ["video.reorderAlbums"](v => 5.44) -> Bool [Video] {
        group_id: Option<Id> = () => {Option},
        album_id: Id = () => {},
        before: Option<Id> = () => {Option},
        after: Option<Id> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct ReorderVideos for ["video.reorderVideos"](v => 5.44) -> Bool [Video] {
        target_id: Option<OwnerId> = () => {Option},
        album_id: Option<Id> = () => {Option},

        owner_id: OwnerId = () => {},
        video_id: Id = () => {},

        before_owner_id: Option<OwnerId> = () => {Option},
        before_video_id: Option<Id> = () => {Option},
        after_owner_id: Option<OwnerId> = () => {Option},
        after_video_id: Option<Id> = () => {Option},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct AddToAlbum for ["video.addToAlbum"](v => 5.44) -> Bool [Video] {
        sized {
            target_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option},
            owner_id: OwnerId = () => {},
            video_id: Id = () => {},
        }
        unsized {
            album_ids: [Id] = (&[][..]) => {Vec},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct RemoveFromAlbum for ["video.removeFromAlbum"](v => 5.44) -> Bool [Video] {
        sized {
            target_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option},
            owner_id: OwnerId = () => {},
            video_id: Id = () => {},
        }
        unsized {
            album_ids: [Id] = (&[][..]) => {Vec},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetAlbumsByVideo for ["video.getAlbumsByVideo"](v => 5.44) -> Collection<Album> [Video] {
        target_id: Option<OwnerId> = () => {Option},
        owner_id: OwnerId = () => {},
        video_id: Id = () => {},
        extended: bool = (true) => {bool},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetComments for ["video.getComments"](v => 5.44, extended => 0) -> Collection<Comment> [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        video_id: Id = () => {},
        need_likes: bool = () => {bool},
        start_comment_id: Option<Id> = () => {Option},
        sort: SortOrder = (SortOrder::Asc) => {AsRef},
        offset: usize = (0) => {},
        count: usize = (20) => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct CreateComment for ["video.createComment"](v => 5.44) -> Id [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            video_id: Id = () => {},
            from_group: bool = () => {bool},
            reply_to_comment: Option<Id> = () => {Option},
            sticker_id: Option<Id> = () => {Option},
        }
        unsized {
            message: str = ("") => {=},
            attachments: [Attachment] = (&[][..]) => {Vec},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct DeleteComment for ["video.deleteComment"](v => 5.44) -> Bool [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        comment_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct RestoreComment for ["video.restoreComment"](v => 5.44) -> Bool [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        comment_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct EditComment for ["video.editComment"](v => 5.44) -> Bool [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            comment_id: Id = () => {},
        }
        unsized {
            message: str = ("") => {=},
            attachments: [Attachment] = (&[][..]) => {Vec},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetTags for ["video.getTags"](v => 5.44) -> Vec<Tag> [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        video_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct PutTag for ["video.putTag"](v => 5.44) -> Id [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            user_id: Id = () => {},
            video_id: Id = () => {},
        }
        unsized {
            tagged_name: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct RemoveTag for ["video.RemoveTag"](v => 5.44) -> Bool [Video] {
        tag_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
        video_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetNewTags for ["video.getNewTags"](v => 5.44) -> Collection<Video> [Video] {
        offset: usize = (0) => {},
        count: usize = (20) => {},
    }
}

request_ref! {
    #[derive(Copy, Eq)]
    struct Report for ["video.report"](v => 5.44) -> Bool [Video] {
        sized {
            owner_id: OwnerId = () => {},
            video_id: Id = () => {},
            reason: ReportReason = (ReportReason::Spam) => {AsRef},
        }
        unsized {
            comment: str = ("") => {=},
            search_query: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Copy, Eq)]
    struct ReportComment for ["video.reportComment"](v => 5.44) -> Bool [Video] {
        owner_id: OwnerId = () => {},
        comment_id: Id = () => {},
        reason: ReportReason = (ReportReason::Spam) => {AsRef},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct GetCatalog for ["video.getCatalog"](v => 5.44, extended => 0) -> Page<CatalogBlock> {
        sized {
            count: usize = (10) => {},
            items_count: usize = (10) => {},
        }
        unsized {
            from: str = ("") => {=},
            filters: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    // TODO: not sure about return type
    struct GetCatalogSection for ["video.getCatalogSection"](v => 5.44, extended => 0) -> Page<CatalogItem> {
        sized {
            section_id: Id = () => {},
            count: usize = (10) => {},
        }
        unsized {
            from: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct HideCatalogSection for ["video.hideCatalogSection"](v => 5.44) -> Bool {
        section_id: Id = () => {},
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

enum_str! { SortOrder {
    Asc = "asc",
    Desc = "desc"
}}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Privacy {
    All = 0,
    Friends = 1,
    FriendsOfFriends = 2,
    OnlyMe = 3,
}

impl AsRef<str> for Privacy {
    fn as_ref(&self) -> &str {
        use self::Privacy::*;
        match *self {
            All => "0",
            Friends => "1",
            FriendsOfFriends => "2",
            OnlyMe => "3",
        }
    }
}

enum_str! { Filter {
    YouTube = "youtube",
    Vimeo = "vimeo",
    Short = "short",
    Long = "long",
}}
