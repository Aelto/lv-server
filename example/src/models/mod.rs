pub mod author;
pub use author::Author;
pub use author::AuthorParams;

pub mod book;
pub use book::Book;
pub use book::BookParams;

pub mod book_content;
pub use book_content::BookContent;

pub mod liked_book;
pub use liked_book::LikedBook;

pub mod library;
pub use library::Library;
pub use library::LibraryParams;

pub mod library_recommendations;
pub use library_recommendations::LibraryRecommendations;
pub use library_recommendations::LibraryRecommendationsParams;

use serde::de::DeserializeOwned;
use surrealdb::opt::PatchOp;
use surrealdb::opt::QueryResult;

use crate::prelude::*;
use std::fmt::Debug;

#[lv_server::async_trait]
pub trait Model
where
  Self: Sized + Serialize + DeserializeOwned + Send + Sync + Debug
{
  fn table() -> &'static str;
  fn m_id(&self) -> Option<&Id>;

  /// A quick function to get the id in string form for when error handling is
  /// not needed.
  fn id(&self) -> &str {
    match self.m_id() {
      Some(repr) => repr.id(),
      None => ""
    }
  }

  /// A quick function to get a ref to the id with the None side already handled
  fn id_res(&self) -> AppResult<&Id> {
    self.m_id().conflict_if_none(Self::table())
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn m_create(self) -> AppResult<Self> {
    let mut item = DB.create(Self::table()).content(self).await?;

    dbg!(&item);
    println!("m_create()");
    unwrap_or_api_error(item.pop())
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn m_delete(&self) -> AppResult<()> {
    if let Some(id) = self.m_id() {
      Self::m_delete_one(id).await?;
    }

    Ok(())
  }

  /// Delete a node using its ID
  async fn m_delete_one(id: &Id) -> AppResult<Self> {
    let item = DB.delete(id.to_thing()?).await?;

    println!("m_delete_one({id})");
    unwrap_or_api_error(item)
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn m_update(self) -> AppResult<Self> {
    if let Some(id) = self.m_id() {
      let item = DB.update(id.to_thing()?).content(self).await?;

      println!("m_update({item:?})");
      unwrap_or_api_error(item)
    } else {
      Ok(self)
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn m_merge_one(id: &Id, merge: impl Serialize + Send) -> AppResult<Self> {
    let item = DB.update(id.to_thing()?).merge(merge).await?;

    println!("m_merge_one({id:?})");
    unwrap_or_api_error(item)
  }

  async fn m_merge(self, merge: impl Serialize + Send) -> AppResult<Self> {
    match self.m_id() {
      Some(id) => Self::m_merge_one(id, merge).await,
      None => Ok(self)
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  /// Add a value to a field, it can be an item in an array or a suffix to a string.
  ///
  async fn m_add_one(id: &Id, field: &str, value: impl Serialize + Send) -> AppResult<Self>
  where
    Self: Debug
  {
    let item = DB
      .update(id.to_thing()?)
      .patch(PatchOp::add(field, value))
      .await?;

    println!("m_add_one({id:?}, {field})");
    unwrap_or_api_error(item)
  }

  async fn m_add(self, field: &str, value: impl Serialize + Send) -> AppResult<Self>
  where
    Self: Debug
  {
    match self.m_id() {
      Some(id) => Self::m_add_one(id, field, value).await,
      None => Ok(self)
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  /// replace a field with a new value, this is particularely useful to update
  /// a field that contains a record-id because of how unreliable it is to
  /// handle [Thing] types, this method guarantees the Thing will be serialized
  /// correctly.
  async fn m_replace_one(id: &Id, field: &str, value: impl Serialize + Send) -> AppResult<Self>
  where
    Self: Debug
  {
    let item = DB
      .update(id.to_thing()?)
      .patch(PatchOp::replace(field, value))
      .await?;

    // todo: temporary until surrealdb fixes Update to no longer return a diff
    // let item = Self::m_find(Where(("id", id))).await?;

    println!("m_replace_one({id:?}, {field})");
    unwrap_or_api_error(item)
  }

  async fn m_replace(self, field: &str, value: impl Serialize + Send) -> AppResult<Self>
  where
    Self: Debug
  {
    match self.m_id() {
      Some(id) => Self::m_replace_one(id, field, value).await,
      None => Ok(self)
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  /// Remove a value from a field, it can be an item from an array or a suffix
  /// from a string.
  ///
  async fn m_remove_one(id: &Id, field: &str, index_or_subfield: &str) -> AppResult<Self> {
    let item = DB
      .update(id.to_thing()?)
      .patch(PatchOp::remove(&format!("{field}/{index_or_subfield}")))
      .await?;

    println!("m_remove_one({id:?}, {field})");
    unwrap_or_api_error(item)
  }

  //////////////////////////////////////////////////////////////////////////////

  async fn m_find<'a, R>(params: impl QueryBuilderInjecter<'a> + Send + 'a) -> AppResult<R>
  where
    R: DeserializeOwned + Debug,
    usize: QueryResult<R>
  {
    let (query, params) = surreal_simple_querybuilder::queries::select("*", Self::table(), params)?;
    let query = DB.query(query).bind(params);
    let items = query.await?.take(0)?;

    Ok(items)
  }

  async fn m_find_one<'a, R>(
    id: &Id, params: impl QueryBuilderInjecter<'a> + Send + 'a
  ) -> AppResult<R>
  where
    R: DeserializeOwned + Debug,
    usize: QueryResult<R>
  {
    let (query, params) = surreal_simple_querybuilder::queries::select("*", "$what", params)?;
    let query = DB.query(query).bind(params);
    let items = query.await?.take(0)?;

    Ok(items)
  }
}

pub(crate) fn unwrap_or_api_error<Opt>(some: Option<Opt>) -> AppResult<Opt> {
  match some {
    Some(v) => Ok(v),
    None => Err("unwrap-on-none".into())
  }
}

#[macro_export]
macro_rules! with_model {
    ($($struct:tt)+) => {
      impl Model for $($struct)+ {
        fn table() -> &'static str {
          &*model
        }

        fn m_id(&self) -> Option<&Id> {
          match self.id.is_empty() {
            false => Some(&self.id),
            true => None
          }
        }
      }
    };
}
