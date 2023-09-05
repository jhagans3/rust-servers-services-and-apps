/*
rust-servers-services-and-apps$ cargo modules generate tree --types --lib -p tutor-db

crate tutor_db
├── mod db_access: pub
├── mod handlers: pub
│   └── mod actix: pub
├── mod models: pub
│   ├── struct Course: pub
│   └── mod actix: pub
├── mod routes: pub
│   └── mod actix: pub
└── mod state: pub
    ├── struct AppState: pub
    └── mod actix: pub


*/

pub mod db_access;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
