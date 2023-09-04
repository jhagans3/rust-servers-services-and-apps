/*
rust-servers-services-and-apps$ cargo modules generate tree --types --lib

crate nodb
├── mod handlers: pub
│   ├── mod actix: pub
│   └── mod axum: pub
├── mod models: pub
│   └── struct Course: pub
├── mod routes: pub
│   ├── mod actix: pub
│   └── mod axum: pub
└── mod state: pub
    ├── struct AppState: pub
    ├── mod actix: pub
    └── mod axum: pub


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
