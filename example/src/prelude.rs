pub use maud::html;
pub use maud::Markup;
pub use maud::Render;

pub use actix_web::web::{delete, get, post, put, scope, Form, Path, Query};
pub use actix_web::HttpResponse;

pub use serde::Deserialize;
pub use serde::Serialize;

pub use crate::db;
pub use crate::extractors::*;
pub use crate::fragments as shared_fragments;
pub use crate::models::*;
pub use crate::AppResult;

pub use crate::page::page;

pub use lv_server::Fragment;
pub use lv_server::Need;
pub use lv_server::View;
pub use lv_server::WithRender;
pub use lv_server::WithRouter;
pub use lv_server::WithScope;
pub use lv_server::WithTrigger;
