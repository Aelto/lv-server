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

    TodoList::render_todo_item(&todo).into_response()
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
    path: Path<String>, Form(form): Form<PostUpdateTodoForm>, data: ApiData
  ) -> HttpResponse {
    let id = path.into_inner();
    let todo = data.update_todo_by_id(id, form.text);

    TodoList::render_todo_item(&todo)
      .join(lv_server::responses::alert("success", &"Item updated"))
      .into_response()
  }
}



use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::sync::LazyLock;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Path;

impl api::Router {
  pub fn once_get(id: &str, handler: crate::Handler) -> String
  {
    let url = id;
    let mut lock = crate::RESOURCES.lock().unwrap();

    if !lock.contains_key(url) {
      lock.insert(url.to_string(), Box::new(handler));
    }

    drop(lock);

    format!("/lv-server/anonymous/{url}")
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
          @for todo in todos {
            (Self::render_todo_item(todo))
          }
        }
      }
    )
  }

  fn render_todo_item(todo: &Todo) -> Markup {
    static URL: LazyLock<String> = lv_server::procedure!(fn endpoint(request: HttpRequest, body: actix_web::web::Payload) -> Pin<Box<dyn Future<Output=HttpResponse>>> {
      Box::pin(async move {
        use actix_web::FromRequest;
        #[derive(Deserialize, Debug)]
        struct F {
          id: String
        }

        let mut body = body.into_inner();
        let form: Form<F> = <Form<F> as FromRequest>::from_request(&request, &mut body).await.unwrap();
        let data = <ApiData as FromRequest>::from_request(&request, &mut body).await.unwrap();

        data.remove_todo_by_id(&form.id);
        TodoList::render(&data.todos()).into_response()
      })
    });

    html!(
      li.fdn.row.items-center
      {
        (todo.text)

          form
            hx-post={(URL.as_str())}
          {
            input type="hidden" name="id" value={(todo.id)};
            button
              {"X"}
          }



        button
          hx-get={(api::get_edit_form::url(&todo.id))}
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
