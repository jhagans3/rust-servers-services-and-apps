/*
rust-servers-services-and-apps$ cargo modules generate tree --types --lib -p nodb

crate nodb
├── mod handlers: pub
│   ├── mod actix: pub
│   ├── mod axum: pub
│   └── mod poem: pub
│       └── struct EzCourseApi: pub
├── mod models: pub
│   ├── struct Course: pub
│   ├── mod actix: pub
│   └── mod poem: pub
│       └── struct Course: pub
├── mod routes: pub
│   ├── mod actix: pub
│   ├── mod axum: pub
│   └── mod poem: pub
└── mod state: pub
    ├── struct AppState: pub
    ├── mod actix: pub
    ├── mod axum: pub
    └── mod poem: pub
        └── struct AppState: pub


*/

pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;
