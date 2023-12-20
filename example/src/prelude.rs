pub use maud::html;
pub use maud::Markup;
pub use maud::Render;

pub use actix_web::web::{delete, get, post, put, scope, Form, Path};
pub use actix_web::HttpResponse;

pub use serde::Deserialize;
pub use serde::Serialize;

pub use crate::db;
pub use crate::fragments;
pub use crate::models::*;
pub use crate::AppResult;

pub use crate::page::page;
