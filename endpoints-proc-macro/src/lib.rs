use std::str::FromStr;

use proc_macro::TokenStream;

mod endpoints;

/// # Example
/// ```rs
/// // can also extend a route with custom config
/// impl ProjectEditForms {
///   fn extend_config_limit(
///     cfg: &mut actix_web::web::ServiceConfig
///   ) -> &mut actix_web::web::ServiceConfig {
///     cfg.app_data(web::FormConfig::default().limit(4096))
///   }
/// }
///
/// lv_server::endpoints!(ProjectEditForms {
///   get_index => GET "{account_id}/{project_slug}"
///   post_create_form => POST "create"
///   post_edit_form extend(extend_config_limit) => POST "{account_id}/{project_slug}"
///   delete_project => DELETE "{account_id}/{project_slug}"
/// });
/// ```
#[proc_macro]
pub fn endpoints(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let (_, model) = endpoints::Router::parse(&content).unwrap_or_default();
  let output = model.to_string();

  // use the following to debug outputs
  // eprintln!("{output}");

  use std::str::FromStr;
  TokenStream::from_str(&output).unwrap_or_default()
}

mod events;

/// # Example
///
/// Declaring the list of events:
/// ```rs
/// lv_server::events!(ProjectEditFormsEvents {
///   Reload "from:body"
/// });
/// ```
///
///
/// Reacting to the events:
/// ```rs
/// div
///   hx-trigger={(ProjectEditFormsEvents::Reload)}
///   hx-get={(api::get_index::url())}
///   hx-target="this"
///   {"This div sends a GET request on this event"}
/// ```
///
///
/// Activating the event from an API endpoint:
/// ```rs
/// impl api::post_index::Router {
///   pub async fn endpoint() -> HttpResponse {
///     ProjectEditForms::render()
///       .join(lv_server::responses::alert("success", "this is a success alert!"))
///       .into_response_with_event(ProjectEditFormsEvents::Reload)
///   }
/// }
/// ```
#[proc_macro]
pub fn events(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let (_, events) = events::Events::parse(&content).unwrap_or_default();
  let output = events.to_string();

  // use the following to debug outputs
  // eprintln!("{output}");

  use std::str::FromStr;
  TokenStream::from_str(&output).unwrap_or_default()
}

mod procedures;

/// Register a procedure, an anonymous endpoint with a unique generated URL that
/// goes with it. The procedure must be a valid [actix_web::Handler] type of function
/// or closure that would normally be registered like so:
/// ```rs
/// fn routes(cfg: &mut actix_web::web::ServiceConfig) {
///   async fn my_procedure() -> actix_web::HttpResponse {
///     todo!()
///   }
///
///   cfg.route("/api/my-path", actix_web::web::post().to(my_procedure));
/// }
/// ```
///
/// ## Limitations
/// Procedures are always POST requests with no path variables, if you need to
/// pass data to the procedure, use a form and the `Form` extractor from Actix.
///
/// # Example
/// The registration is done at compile-time, it is completely fine to declare
/// a procedure in a for loop or frequently called function like so:
///
/// > After registering procedures, you will have to use two other macros to
/// > collect & register the endpoint related to those procedures. Refer to
/// > the [register_procedures] macro, and [collect_procedures].
/// ```rs
/// fn render_todo_item(todo: &Todo) -> Markup {
///   #[derive(serde::Deserialize)]
///   struct F {
///     id: String
///   }
///
///   // 👇 register the procedure, it's a regular actix Handler function.
///   let url = lv_server::procedure!(
///     async |Form(form): Form<F>, data: crate::app_data::ApiData| {
///       data.remove_todo_by_id(&form.id);
///       TodoList::render(&data.todos()).into_response()
///     }
///   );
///
///   html!(
///     li.fdn.row.items-center
///     {
///       (todo.text)
///
///       form hx-post={(url)} // 👈 you can use the url variable
///       {
///         input type="hidden" name="id" value={(todo.id)};
///         button {"X"}
///       }
///     }
///   )
/// }
/// ```
#[proc_macro]
pub fn procedure(input: TokenStream) -> TokenStream {
  let uuid = nanoid::nanoid!();
  let path = format!("/lvsrv/procs/{uuid}");

  let input = proc_macro2::TokenStream::from(input);
  let output = quote::quote! {
    { // it emits a code block
      lv_server::deps::inventory::submit! {
        crate::LvEndpoint::new(|cfg: &mut actix_web::web::ServiceConfig| {
          cfg.route(
            #path,
            web::post().to(#input)
          );
        })
      };

      let path = #path; // and returns the path as a &'static str

      path
    }
  };

  // eprintln!("{output}");

  output.into()
}

/// Once you've declared procedures using the [procedure] macro, you must register
/// the endpoint related to those procedures. To do so, you must add a configure
/// block to the actix_web setup code.
///
/// # Example
/// > After declaring & registering the procedures, you must collect them using
/// > the [collect_procedures] macro.
/// ```rs
/// HttpServer::new(move || {
///   App::new()
///     .app_data(actix_web::web::Data::clone(&app_data))
///     .configure(routes)
///     .configure(|cfg| { // 👈 like so 👇
///       lv_server_endpoints_proc_macro::register_procedures!(cfg);
///     })
/// })
/// ```
#[proc_macro]
pub fn register_procedures(input: TokenStream) -> TokenStream {
  let input = proc_macro2::TokenStream::from(input);
  let output = quote::quote! {
    for endpoint in inventory::iter::<crate::LvEndpoint> {
      (endpoint.handler)(#input);
    }
  };

  output.into()
}

/// The last step to using procedures is to collect them, to do so call this macro
/// anywhere in `main.rs`
///
/// ```rs
/// // 👇
/// lv_server::collect_procedures!();
///
/// #[tokio::main]
/// async fn main() {
///   // ...
/// }
/// ```
#[proc_macro]
pub fn collect_procedures(_: TokenStream) -> TokenStream {
  let output = quote::quote! {
    /// This type is generated by the `lv_server::collect_procedures!()` macro.
    /// It is used, in conjunction with the `inventory` crate to register the
    /// endpoint & procedures from anywhere in the source code.
    ///
    /// The `handler` field holds pointers to functions that can be used with
    /// Actix' `configure` method, to configure new endpoint & services.
    pub struct LvEndpoint {
      pub handler: fn(cfg: &mut lv_server::deps::actix_web::web::ServiceConfig)
    }

    impl LvEndpoint {
      pub const fn new(handler: fn(cfg: &mut lv_server::deps::actix_web::web::ServiceConfig)) -> Self {
        Self { handler }
      }
    }

    lv_server::deps::inventory::collect!(LvEndpoint);
  };

  output.into()
}

mod prelude {

  pub use nom::bytes::complete::{tag, take_until1, take_while, take_while1};
  pub use nom::error::ParseError;
  pub use nom::multi::many0;
  pub use nom::IResult;

  pub fn trim(i: &str) -> IResult<&str, &str> {
    take_while(|c| c == ' ' || c == '\n' || c == '\r')(i)
  }
}
