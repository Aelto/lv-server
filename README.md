# LV-Server
The `lv-server` crate acts as a framework for writing reactive web pages entirely
with backend (rust) code thanks to [HTMX](https://htmx.org/), [Actix](https://actix.rs/), and [Maud](https://maud.lambda.xyz/).

- Actix web serves as the HTTP server
- Maud does the backend HTML templating using a simple macro-like syntax
- HTMX allows for dynamic user interfaces that communicate via standard HTTP requests and custom events

One of the main reasons to use such a framework is to move all of the templating
logic from the frontend to the backend, meaning that your templates can now have
access to elements that would be otherwise inaccessible to the frontend code unless
a REST API is setup. By doing so the need for javascript code is eliminated completely (outside of HTMX.js itself),
saving us from creating such an API or from thinking about serializing our models
from/to JSON, and allows us to profit from sweet compile errors right in our in HTML templates.

However combining these three libraries without preparation would lead to a somewhat
boilerplate-y experience due to HTMX' way of using API endpoints for most user actions,
having to define the endpoints, remembering the routes, avoiding typos, and simply organizing
a project with so many URLs can be complicated. `lv-server` aims to streamline most
of this boilerplate using a combination of macros and traits so you get to focus on
writing your interface's logic while profiting from compile time errors to prevent
the common mistakes like typos in endpoint URLs for example.


## Using lv-server
> [A complete example project that showcases a dynamic ToDo list is available](lv-server/examples/todo-list/).

User interfaces made with `lv-server` are composed of two main elements:
### UI: Views
_[view this code in the example project](lv-server/examples/todo-list/views/_home/mod.rs)_
```rs
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
```

[Views](./lv-server/src/view.rs), are the accessible pages of the website.
A single view can have multiple routes/endpoints, however for smaller but more
dynamic changes it is recommended to define fragments on the view:
```rs
impl lv_server::View<(fragments::TodoList, fragments::AddTodoForm)> for ViewHome {}
```

Linking a fragment to a view tells lv-server to automatically setup an API endpoint
for that fragment as soon as the view itself is setup. You don't have to worry about
how or when to declare the URLs of your fragments as long as you link them to a view.

_[view this code in the example project](lv-server/examples/todo-list/main.rs)_
```rs
// setting up a view in the main Actix app:
fn routes(cfg: &mut actix_web::web::ServiceConfig) {
  use lv_server::View;

  // this sets up the View itself, but also any fragment it may have:
  views::ViewHome::router(cfg);
}
```

### UI: Fragments
_[view this code in the example project](lv-server/examples/todo-list/views/_home/fragments/add_todo_form.rs)_
```rs
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
```

Fragments are similar to Views except
1) they don't have children fragments like views
2) events can be defined with them.
3) their endpoints are automatically given a prefix to avoid users hitting them by mistakes

### The `endpoints!` macro
_[view this code in the example project](lv-server/examples/todo-list/views/_home/fragments/todo_list.rs)_
```rs
lv_server::endpoints!(TodoList {
  get_index => GET "/"
  get_todo => GET "/todos/{index}"

  delete_todo => DELETE "/todos/{index}"

  get_edit_form => GET "/todos/{index}/edit"
  post_update_todo => POST "/todos/{index}"
});
```

The `endpoints!` macro saves you from a lot of boilerplate and generates nestings
of modules and structs so everything can be traversed easily but also safely thanks
to compile time errors.

_[view this code in the example project](lv-server/examples/todo-list/views/_home/fragments/todo_list.rs)_
```rs
impl api::delete_todo::Router {
  pub async fn endpoint(path: Path<usize>, data: ApiData) -> HttpResponse {
    data.remove_todo_by_index(path.into_inner());

    TodoList::render(&data.todos()).into_response()
  }
}
```
```rs
button
  hx-delete={(api::delete_todo::url(&index.to_string()))}
  hx-confirm={"Delete todo '"(todo.text)"'?"}
  {"X"}
```

For every route that's defined in the `endpoints!` macro, a module with the name
of that route is created inside the `api` module, for example the `api::delete_todo`
module above.

In the module a `struct Router` is created with a missing implementation for a 
`pub async fn endpoint` function. Not implementing the function will cause the
macro to throw an error, and that function can accept any parameter or return
anything the `actix_web` crate would accept as a regular endpoint.


You may also notice that once an endpoint & its route is defined in the macro there
is no need to remember that route anymore as now everything can be done through
the static functions like: `api::delete_todo::url(index: &str)`. Saving you from
the 404 errors from typos, or allows you to change the route without worrying about
breaking a form in some long forgotten fragment.

### Utilities
`lv-server` makes it mandatory to include a `X-LVSERVER-REQ` header to any non GET request. Without it any request to a view or fragment that isn't a GET will
become a 404. The easiest solution to tell HTMX to include the header to its request is to use the [`hx-headers`](https://htmx.org/attributes/hx-headers/) attribute to a parent node, for example adding the attribute to the page's body inside our maud templates:
```rs
body hx-headers={"{ \"X-LVSERVER-REQ\": \""(nanoid::nanoid!())"\" }"} {(content)}
```

Note that this is one of the many layers to protect against CSRF, yet it's a simple
and efficient protection that it'd be a shame not to have it, hence its mandatory status.
_[example on how to add it to the page using maud](lv-server/examples/todo-list/page.rs)_

---

A [ExtMaudMarkup](lv-server/src/ext_maud.rs) extension trait is offered by the crate
to simplify the common operations between maud's Markup, lv-server events, and actix
HttpResponse.


_[view this code in the example project](lv-server/examples/todo-list/views/_home/fragments/add_todo_form.rs.rs)_
```rs
TodoList::render_todo_item(&todo, index)
  .join(lv_server::responses::alert("success", &"Item updated"))
  .into_response()
```

---

A basic alert/popup/toast system can be implemented with `lv-server`. Adding the
following element anywhere on the page is enough:
```html
<div id="lv-alert" hidden></div>
```

Then any lv-server endpoint can trigger an alert using the [lv_server::responses::alert](lv-server/src/responses.rs) function:
```rs
lv_server::responses::alert("success", &"Item updated").into_response()
```
