use super::users::NameCase;
use super::api::Collection;

request_lt! {
    struct Get for ["users.get"](v => 5.44, extended => 0) -> Collection<Post> {
        sized {
            owner_id: i64 = () => {},
            name_case: NameCase = (NameCase::Nominative) => {AsRef},
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

pub enum Filter {
    Owner,
    Others,
    All,
    Suggests,
}

impl AsRef<str> for Filter {
    fn as_ref(&self) -> &str {
        use self::Filter::*;
        match *self {
            Owner => "owner",
            Others => "others",
            All => "all",
            Suggests => "suggests",
        }
    }
}
