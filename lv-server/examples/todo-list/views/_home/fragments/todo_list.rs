use actix_web::web::Path;

use crate::prelude::*;

pub struct TodoList;

lv_server::events!(TodoListEvents {
  Reload "from:body"
});

impl lv_server::Fragment<TodoListEvents, api::Router> for TodoList {
  const ID: &'static str = "TodoList";
}

lv_server::endpoints!(TodoList {
  get_index => GET "/"
  get_todo => GET "/todos/{index}"

  delete_todo => DELETE "/todos/{index}"

  get_edit_form => GET "/todos/{index}/edit"
  post_update_todo => POST "/todos/{index}"
});

impl api::get_index::Router {
  pub async fn endpoint(data: ApiData) -> HttpResponse {
    TodoList::render(&data.todos()).into_response()
  }
}

impl api::delete_todo::Router {
  pub async fn endpoint(path: Path<usize>, data: ApiData) -> HttpResponse {
    data.remove_todo_by_index(path.into_inner());

    TodoList::render(&data.todos()).into_response()
  }
}

impl api::get_todo::Router {
  pub async fn endpoint(path: Path<usize>, data: ApiData) -> HttpResponse {
    let index = path.into_inner();
    let todo = data.todos().remove(index);

    TodoList::render_todo_item(&todo, index).into_response()
  }
}

impl api::get_edit_form::Router {
  pub async fn endpoint(path: Path<usize>, data: ApiData) -> HttpResponse {
    let index = path.into_inner();
    let todo = data.todos().remove(index);

    TodoList::render_todo_edit_form(&todo, index).into_response()
  }
}

#[derive(Deserialize)]
pub struct PostUpdateTodoForm {
  text: String
}

impl api::post_update_todo::Router {
  pub async fn endpoint(
    path: Path<usize>, Form(form): Form<PostUpdateTodoForm>, data: ApiData
  ) -> HttpResponse {
    let index = path.into_inner();
    let todo = data.update_todo_by_index(index, form.text);

    TodoList::render_todo_item(&todo, index)
      .join(lv_server::responses::alert("success", &"Item updated"))
      .into_response()
  }
}

impl TodoList {
  pub fn render(todos: &Vec<Todo>) -> Markup {
    html!(
      .fdn.block.col
        hx-trigger={(TodoListEvents::Reload)}
        hx-get={(api::get_index::url())}
        hx-target="this"
      {
        .fdn.title {"Your todos"}
        ul.fdn.col {
          @for (index, todo) in todos.iter().enumerate() {
            (Self::render_todo_item(todo, index))
          }
        }
      }
    )
  }

  fn render_todo_item(todo: &Todo, index: usize) -> Markup {
    html!(
      li.fdn.row.items-center
      {
        (todo.text)

        button
          hx-delete={(api::delete_todo::url(&index.to_string()))}
          hx-confirm={"Delete todo '"(todo.text)"'?"}
          {"X"}

        button
          hx-get={(api::get_edit_form::url(&index.to_string()))}
          hx-target="closest li"
          hx-swap="outerHTML"
          {"✏️"}
      }
    )
  }

  fn render_todo_edit_form(todo: &Todo, index: usize) -> Markup {
    html!(
      form
        hx-post={(api::post_update_todo::url(&index.to_string()))}
        hx-target="this"
        hx-swap="outerHTML"
      {
        input name="text" value={(todo.text)};

        button
          hx-get={(api::get_todo::url(&index.to_string()))}
          {"cancel"}

        input type="submit" value="save";
      }
    )
  }
}
