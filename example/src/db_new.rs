use crate::prelude::*;
use surrealdb::Surreal;

pub type SurrealClient<T> = Surreal<T>;
pub static DB: Surreal<surrealdb::engine::local::Db> = Surreal::init();

pub async fn connect(
  _address: &str, _username: &str, _password: &str, namespace: &str, database: &str
) -> AppResult<()> {
  DB.connect::<surrealdb::engine::local::Mem>(()).await?;
  DB.use_ns(namespace).use_db(database).await?;

  Ok(())
}
