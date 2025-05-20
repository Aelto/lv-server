use crate::prelude::*;

pub struct AddTodoForm;

impl lv_server::Fragment<(), api::Router> for AddTodoForm {
  const ID: &'static str = "AddTodoForm";
}

lv_server::endpoints!(AddTodoForm {
  post_add_todo => POST "/todos"
});

#[derive(Deserialize)]
pub struct PostAddTodoForm {
  text: String
}

impl api::post_add_todo::Router {
  pub async fn endpoint(Form(form): Form<PostAddTodoForm>, data: ApiData) -> HttpResponse {
    if form.text.trim().is_empty() {
      return AddTodoForm::render()
        .join(lv_server::responses::alert(
          "error",
          &"You can't add an empty todo"
        ))
        .into_response();
    }

    data.add_todo(form.text);

    AddTodoForm::render().into_response_with_event(super::TodoListEvents::Reload)
  }
}

impl AddTodoForm {
  pub fn render() -> Markup {
    html!(
      form.fdn.row
        hx-post={(api::post_add_todo::url())}
        hx-target="this"
        hx-swap="outerHTML"
      {
        input name="text" placeholder="Todo's text" {}
        input type="submit" value="Add";
      }
    )
  }
}
