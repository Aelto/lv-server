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
}

pub fn read<T>(key: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
where
  T: DeserializeOwned
{
  match CELL.get() {
    Some(mutex) => match mutex.lock() {
      Ok(cell) => match cell.borrow().get(key) {
        Some(value) => Ok(serde_json::from_slice(value.as_ref())?),
        None => Ok(None)
      },
      Err(e) => Err(Box::new(e))
    },
    None => Ok(None)
  }
}

pub fn write(key: String, value: &impl Serialize) -> Result<(), Box<dyn std::error::Error>> {
  match CELL.get() {
    Some(mutex) => match mutex.lock() {
      Ok(cell) => {
        cell
          .borrow_mut()
          .insert(key, serde_json::to_string(&value)?);

        Ok(())
      }
      Err(e) => Err(Box::new(e))
    },
    None => Ok(())
  }
}
