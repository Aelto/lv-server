use std::fmt::Display;

use crate::prelude::*;

use super::Endpoint;

#[derive(Debug, Clone, Default)]
pub struct Router {
  name: String,
  endpoints: Vec<Endpoint>,

  /// defaults to true, implements the WithRouter trait for the Fragment.
  /// There is currently no way to disable it but it might be useful later on
  impl_router: bool
}

impl Router {
  pub fn parse(i: &str) -> IResult<&str, Self> {
    let (i, _) = trim(i)?;
    let (i, name) = take_until1("{")(i)?;
    let (i, _) = tag("{")(i)?;
    let (i, endpoints) = many0(Endpoint::parse)(i)?;
    let (i, _) = nom::bytes::complete::take_until("}")(i)?;

    Ok((
      i,
      Self {
        endpoints,
        name: name.trim().to_owned(),
        impl_router: true
      }
    ))
  }

  /// Emits a router that combines all of the endpoint routers
  fn emit_router(&self) -> proc_macro2::TokenStream {
    let configures: Vec<proc_macro2::TokenStream> = self
      .endpoints
      .iter()
      .map(|e| {
        let module = quote::format_ident!("{}", e.name);

        quote::quote!(
          cfg.configure(#module::Router::router);
        )
      })
      .collect();

    quote::quote!(
      pub struct Router;
      impl lv_server::WithRouter for Router {
        fn router(cfg: &mut actix_web::web::ServiceConfig) {
          #(#configures)*
        }
      }
    )
  }

  fn emit_router_impl(&self) -> proc_macro2::TokenStream {
    match self.impl_router {
      true => {
        let name = quote::format_ident!("{}", self.name);

        quote::quote!(
          impl lv_server::WithRouter for super::#name
            {
            fn router(cfg: &mut actix_web::web::ServiceConfig) {
              cfg.configure(Router::router);
            }
          }
        )
      }
      false => {
        quote::quote!()
      }
    }
  }
}

impl Display for Router {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use proc_macro2::TokenStream;
    let endpoint_mods: Vec<TokenStream> = self
      .endpoints
      .iter()
      .map(|endpoint| endpoint.emit(&self.name))
      .collect();

    let router = self.emit_router();
    let router_impl = self.emit_router_impl();

    let output = quote::quote! {
      pub mod api {
        #router
        #router_impl

        #(#endpoint_mods)*
      }
    };

    write!(f, "{output}")
  }
}
