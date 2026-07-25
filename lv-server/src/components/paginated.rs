use std::marker::PhantomData;

pub struct Paginated<I>(PhantomData<I>);

impl<I> Paginated<I>  where I: ComponentPagination{
  pub fn router(cfg: &mut actix_web::web::ServiceConfig) {
    I::router(cfg);
  }

  pub fn render(items: &[I]) -> maud::Markup where Self: Sized{
    I::render(items)
  }
}

#[derive(serde::Deserialize)]
pub struct LoadMoreForm {
  pub after: Option<String>
}

pub trait ComponentPagination {
  const COMPONENT_ID: &'static str;

  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route(&Self::api_post_load_more_url(), Self::endpoint_load_more(actix_web::web::post()));
  }

  fn endpoint_load_more(route: actix_web::Route) -> actix_web::Route;
  fn id(&self) -> &str;

  fn api_post_load_more_url() -> String {
    format!("/frg/lvs/{}/load-more", Self::COMPONENT_ID)
  }

  fn classes_item<'a>() -> &'a str {""}
  fn classes_item_list<'a>() -> &'a str {""}
  fn classes_load_more_button<'a>() -> &'a str {""}

  fn render(items: &[Self]) -> maud::Markup where Self: Sized{
    maud::html!(
      // forced to do that to avoid a bug in maud's macro
      @for i in items.iter().map(|i| i.render_item()) {
        (i)
      }

      (Self::render_load_more_button(items.last()))
    )
  }

  fn render_item(&self) -> maud::Markup;

  fn render_load_more_button(last: Option<&Self>) -> maud::Markup {
    maud::html!(
      form
        hx-post={(Self::api_post_load_more_url())}
        hx-target="this"
        hx-swap="outerHTML"
      {
        @if let Some(last) = last {
          input name="after" type="hidden" value=(last.id());
        }
        button type="submit" class={(Self::classes_load_more_button())} {"Load more"}
      }
    )
  }
}

// pub struct Paginated<API, STYLES, RENDER>(PhantomData<API>, PhantomData<STYLES>, PhantomData<RENDER>);

// pub trait PaginatedApi: Sized {
//   /// A unique ID for this implementation of [WithPagination], once combined with
//   /// [Paginated] this ID ensures that all endpoints of this implementation do not
//   /// with conflict with other implementations.
//   const ID: &'static str;
//   fn load_more_endpoint(route: actix_web::Route) -> actix_web::Route;
// }

// impl PaginatedStyles for () {}
// pub trait PaginatedStyles {
//   fn classes_item<'a>() -> &'a str {""}
//   fn classes_item_list<'a>() -> &'a str {""}
//   fn classes_load_more_button<'a>() -> &'a str {""}
// }

// pub trait PaginatedRender {
//   fn id(&self) -> impl maud::Render;

//   /// Markup for one item of the list
//   fn render_item(&self) -> maud::Markup;
// }

// impl<API, STYLES, RENDERING> Paginated<API, STYLES, RENDERING>
//   where API: PaginatedApi,
//     STYLES: PaginatedStyles,
//     RENDERING: PaginatedRender
// {


//   fn post_load_more_url() -> String {
//     format!("/frg/lvs/{}/load-more", API::ID)
//   }

//   fn render(items: &[RENDERING]) -> maud::Markup where {
//     maud::html!(
//       // forced to do that to avoid a bug in maud's macro
//       @for i in items.iter().map(|i| i.render_item()) {
//         (i)
//       }

//       @if let Some(last) = items.last() {
//         (Self::render_load_more_button(last))
//       }
//     )
//   }

//   fn render_load_more_button(last: &RENDERING) -> maud::Markup {
//     maud::html!(
//       form
//         hx-post={(Self::post_load_more_url())}
//         hx-target="this"
//         hx-swap="outerHTML"
//       {
//         input type="hidden" value=(last.id());
//         button class={(STYLES::classes_load_more_button())} {"Load more"}
//       }
//     )
//   }
// }
