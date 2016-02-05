use users::UserOptionField;
use api::{Bool, Collection, Timestamp, Id, LikesCount, OwnerId};

#[cfg(feature = "unstable")]
include!("wall.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/wall.rs"));

request_ref! {
    struct Get for ["wall.get"](v => 5.44, extended => 0) -> Collection<WallPost> {
        sized {
            owner_id: OwnerId = () => {},
            filter: Filter = (Filter::All) => {AsRef},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            domain: str = ("") => {=},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_ref! {
    struct Post for ["wall.post"](v => 5.44) -> PostId [Wall] {
        sized {
            owner_id: OwnerId = () => {},
            friend_only: bool = (true) => {bool},
            from_group: bool = (false) => {bool},
            signed: bool = (false) => {bool},
            publish_date: Timestamp = (0) => {},
            lat: f32 = () => {},
            long: f32 = () => {},
            place_id: Id = () => {},
            post_id: Id = () => {},
        }
        unsized {
            message: str = ("") => {=},
            services: str = ("") => {=},
        }
    }
}

enum_str! { Filter {
    Owner = "owner",
    Others = "others",
    All = "all",
    Suggests = "suggests",
}}
