use crate::prelude::*;

pub struct BookViewEditToggle;

impl lv_server::Fragment<()> for BookViewEditToggle {
  const ID: &'static str = "BookViewToggle";
}
impl WithRouter for BookViewEditToggle {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    BookViewEditToggle::fragment_route(cfg, "{library_id}/{book_id}", get().to(get_index));
    async fn get_index(
      Need(LibraryBookPathExt(_lib, book)): Need<LibraryBookPathExt>
    ) -> HttpResponse {
      lv_server::responses::html(BookViewEditToggle::render(&book))
    }

    BookViewEditToggle::fragment_route(cfg, "{library_id}/{book_id}/edit", get().to(get_edit_form));
    async fn get_edit_form(
      Need(LibraryBookPathExt(_lib, book)): Need<LibraryBookPathExt>
    ) -> HttpResponse {
      lv_server::responses::html(BookViewEditToggle::render_edit_form(&book))
    }

    BookViewEditToggle::fragment_route(cfg, "{library_id}/{book_id}", put().to(put_library_book));
    async fn put_library_book(
      Need(LibraryBookPathExt(_lib, mut book)): Need<LibraryBookPathExt>, Form(form): Form<Book>
    ) -> HttpResponse {
      book.content = form.content;
      book.title = form.title;
      book.update().unwrap();

      let view = BookViewEditToggle::render(&book);
      let html = lv_server::responses::html(view);

      super::BookListEvents::Reload.trigger(html)
    }
  }
}

impl BookViewEditToggle {
  pub fn render(book: &Book) -> Markup {
    html!(
      div.document hx-swap="outerHTML" hx-target="this" {
        h1 {(book.title)}
        div.actions {
          button hx-get={(Self::url(&book.fk_library))"/"(book.id)"/edit"} {"Edit book"}
        }

        pre {(book.content)}
      }
    )
  }

  pub fn render_edit_form(book: &Book) -> Markup {
    html!(
      div.document hx-swap="outerHTML" hx-target="this" {
        form hx-put={(Self::url(&book.fk_library))"/"(book.id)} {
          div {
            button {"save"}
            button hx-get={(Self::url(&book.fk_library))"/"(book.id)} {"cancel"}
          }
          input name="title" value={(book.title)};
          textarea name="content" {(book.content)}
        }
      }
    )
  }
}
