pub use crate::prelude::*;

pub mod fragments;

pub struct ViewHome;

impl lv_server::View<(fragments::TodoList, fragments::AddTodoForm)> for ViewHome {}

lv_server::endpoints!(ViewHome as view {
  get_index => GET "/"
});

impl api::get_index::Router {
  async fn endpoint(data: ApiData) -> HttpResponse {
    page(ViewHome::render(data)).into_response()
  }
}

impl ViewHome {
  fn render(data: ApiData) -> Markup {
    html!(
      .fdn.col.justify-center.items.center {
        (fragments::TodoList::render(&data.todos()))
        (fragments::AddTodoForm::render())
      }
    )
  }
}
