use crate::prelude::*;

mod crud;
pub mod partials;
pub use partials::BookParams;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Book {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub title: String,
  pub content: String,
  pub library: ForeignKey<Library, Id>,
  pub created_at: chrono::DateTime<chrono::Utc>
}

surreal_simple_querybuilder::model!(Book {
  id,
  pub title,
  pub content,
  pub library,
  pub created_at
});
pub use schema::model;
crate::with_model!(Book);

impl Book {
  pub async fn is_author(&self, author_id: &Id) -> AppResult<bool> {
    let Some(lib) = Library::find_by_id(self.library.fk(), LibraryParams::None).await? else {
      return Ok(false);
    };

    Ok(lib.author.fk().eq(author_id))
  }
}

impl maud::Render for Book {
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
      div.title {(self.title)}
      div.content {(maud::PreEscaped(rendered_markdown))}
    )
  }
}

impl IntoKey<Id> for Book {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}
