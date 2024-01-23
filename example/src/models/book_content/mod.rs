use crate::prelude::*;

mod crud;

/// The content of a book is separated in a different record to give control on
/// whether the content is loaded with the book's data or not.
///
/// [BookContent] has no foreign field to the [Book] as it uses the same uuid as
/// the book so switch back-and-forth should be as easy as swapping the `tb` from
/// the record ids.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BookContent {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub content: String
}

surreal_simple_querybuilder::model!(BookContent {
  id,

  pub content
});
pub use schema::model;
crate::with_model!(BookContent);

impl IntoKey<Id> for BookContent {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}

impl BookContent {
  pub fn custom_id(book_id: &Id) -> Id {
    Id::new_thing(model.to_string(), book_id.id())
  }

  pub async fn set_by_book_id(book_id: &Id, content: String) -> AppResult<Self> {
    let item = Self::find_by_book_id(book_id).await?;

    match item {
      Some(mut book_content) => {
        book_content.content = content;
        book_content.update().await
      }
      None => {
        Self {
          content,
          ..Default::default()
        }
        .create(book_id)
        .await
      }
    }
  }
}

impl maud::Render for BookContent {
  fn render(&self) -> maud::Markup {
    let rendered_markdown = {
      use ammonia::clean;
      use pulldown_cmark::html::push_html;
      use pulldown_cmark::Options;
      use pulldown_cmark::Parser;

      let mut options = Options::empty();
      options.insert(Options::ENABLE_TABLES);
      options.insert(Options::ENABLE_STRIKETHROUGH);
      options.insert(Options::ENABLE_TASKLISTS);

      let md_parse = Parser::new_ext(&self.content, options);
      let mut unsafe_html = String::new();
      push_html(&mut unsafe_html, md_parse);

      let safe_html = clean(&*unsafe_html);

      safe_html
    };

    html!(
      div.content {(maud::PreEscaped(rendered_markdown))}
    )
  }
}
