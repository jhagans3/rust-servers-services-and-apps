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

pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;
