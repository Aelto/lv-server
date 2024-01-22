pub use maud::html;
pub use maud::Markup;
pub use maud::Render;

pub use actix_web::web::{delete, get, post, put, scope, Form, Path, Query};
pub use actix_web::HttpResponse;

pub use serde::Deserialize;
pub use serde::Serialize;

pub use crate::db;
pub use crate::db_new::DB;
pub use crate::dev;
pub use crate::extractors::*;
pub use crate::fragments as shared_fragments;
pub use crate::models::*;

use crate::result;
pub use result::AppError;
pub use result::AppResponse;
pub use result::AppResult;
pub use result::TemplateResponse;

use crate::exts;
pub use exts::foreign::*;
pub use exts::option::*;
pub use exts::result::*;
pub use exts::vec::*;

use crate::types;
pub use types::*;

pub use crate::page::page;

pub use lv_server::Fragment;
pub use lv_server::Need;
pub use lv_server::View;
pub use lv_server::WithRender;
pub use lv_server::WithRouter;
pub use lv_server::WithScope;
pub use lv_server::WithTrigger;

pub use surreal_simple_querybuilder::prelude::*;
pub use surreal_simple_querybuilder::wjson;
pub use surrealdb::sql::thing;
pub use surrealdb::sql::Thing;
