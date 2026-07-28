use crate::app_data::ApiData;
use actix_web::{web::Form, HttpResponse};
use lv_server::{components::paginated, ExtMaudMarkup};

pub type PaginatedFakeItem = paginated::Paginated<crate::app_data::FakeItem>;

impl paginated::ComponentPagination for crate::app_data::FakeItem {
  const COMPONENT_ID: &'static str = "PaginatedTodos";

  fn endpoint_load_more(route: actix_web::Route) -> actix_web::Route {
    async fn endpoint(data: ApiData, Form(form): Form<paginated::LoadMoreForm>) -> HttpResponse {
      let next_five = match form.after {
        Some(after) => data.find_fake_items_after(&after),
        None => data.find_fake_items_after("-1")
      };

      paginated::ComponentPagination::render(&next_five).into_response()
    }

    route.to(endpoint)
  }

  fn id(&self) -> &str {
    &self.id
  }

  fn render_item(&self) -> maud::Markup {
    maud::html!(
      item {
        "id = " (self.id)
      }
    )
  }
}
