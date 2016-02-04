use api::{Id, Bool};

request_ref! {
    #[derive(Eq, Copy)]
    struct Get for ["storage.get"](v => 5.44) -> String {
        sized {
            user_id: Id = () => {},
            global: bool = () => {bool},
        }
        unsized {
            key: str = ("") => {=},
            keys: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Set for ["storage.set"](v => 5.44) -> Bool {
        sized {
            user_id: Id = () => {},
            global: bool = () => {bool},
        }
        unsized {
            key: str = ("") => {=},
            value: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetKeys for ["storage.getKeys"](v => 5.44) -> Vec<String> {
        user_id: Id = () => {},
        global: bool = () => {bool},
        offset: usize = (0) => {},
        count: usize = (100) => {},
    }
}
