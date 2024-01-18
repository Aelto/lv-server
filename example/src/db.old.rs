use crate::prelude::*;

use std::{
  cell::RefCell,
  collections::HashMap,
  sync::{Mutex, OnceLock}
};

use serde::de::DeserializeOwned;
use serde::Serialize;

pub type DbKey = String;
pub type DbValue = String;

static CELL: OnceLock<Mutex<RefCell<HashMap<DbKey, DbValue>>>> = OnceLock::new();

pub fn init() {
  CELL.get_or_init(|| Mutex::new(RefCell::new(HashMap::new())));

  let author = crate::models::Author {
    handle: "SignedUser".to_owned(),
    id: String::new()
  };
  author.add().unwrap();
  let author = crate::dev::signed_user();

  let lib = crate::models::Library {
    title: "Nook".to_owned(),
    ..Default::default()
  };
  lib.add(author.id).unwrap();

  for lib in crate::models::Library::find_all().unwrap() {
    let book = crate::models::Book {
      title: "lorem".to_owned(),
      ..Default::default()
    };

    book.add(lib.id).unwrap();
  }
}

pub fn read<T>(key: &str) -> AppResult<Option<T>>
where
  T: DeserializeOwned
{
  match CELL.get() {
    Some(mutex) => match mutex.lock() {
      Ok(cell) => match cell.borrow().get(key) {
        Some(value) => Ok(serde_json::from_slice(value.as_ref())?),
        None => Ok(None)
      },
      Err(_) => Err(AppError::InternalServerError("DB:CELL.get()"))
    },
    None => Ok(None)
  }
}

pub fn write(key: String, value: &impl Serialize) -> AppResult<()> {
  match CELL.get() {
    Some(mutex) => match mutex.lock() {
      Ok(cell) => {
        cell
          .borrow_mut()
          .insert(key, serde_json::to_string(&value)?);

        Ok(())
      }
      Err(_) => Err(AppError::InternalServerError("DB:write"))
    },
    None => Ok(())
  }
}
