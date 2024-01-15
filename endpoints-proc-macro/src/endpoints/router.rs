use std::fmt::Display;

use crate::prelude::*;

use super::Endpoint;

#[derive(Debug, Clone, Copy)]
pub enum RouterType {
  Fragment,
  View
}
impl Default for RouterType {
  fn default() -> Self {
    Self::Fragment
  }
}

#[derive(Debug, Clone, Default)]
pub struct Router {
  name: String,
  endpoints: Vec<Endpoint>,

  /// controls what kind of endpoint it emits. By default it emits endpoints
  /// with implementations that fit Fragments, it can be tweaked to emit View
  /// endpoints using the `as View` syntax after the struct name and before the
  /// `{`
  router_type: RouterType,

  /// defaults to true, implements the WithRouter trait for the Fragment.
  /// There is currently no way to disable it but it might be useful later on
  impl_router: bool
}

impl Router {
  pub fn parse(i: &str) -> IResult<&str, Self> {
    let (i, _) = trim(i)?;
    let (i, name_and_type) = take_until1("{")(i)?;
    let (_, (name, router_type)) = Self::parse_name_and_type(name_and_type)?;
    let (i, _) = tag("{")(i)?;
    let (i, endpoints) = many0(Endpoint::parse)(i)?;
    let (i, _) = nom::bytes::complete::take_until("}")(i)?;

    Ok((
      i,
      Self {
        endpoints,
        name: name.trim().to_owned(),
        router_type,
        impl_router: true
      }
    ))
  }

  fn parse_name_and_type(i: &str) -> IResult<&str, (&str, RouterType)> {
    let (i, _) = trim(i)?;
    let (i, name) = take_while1(|c| c != ' ')(i)?;
    let (i, some_router_type) = nom::combinator::opt(Self::parse_router_type)(i.trim())?;

    Ok((i, (name, some_router_type.unwrap_or(RouterType::Fragment))))
  }

  fn parse_router_type(i: &str) -> IResult<&str, RouterType> {
    let (i, _) = tag("as")(i)?;

    match i.trim() {
      "View" | "view" => Ok((i, RouterType::View)),
      _ => Ok((i, RouterType::Fragment))
    }
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
      .map(|endpoint| endpoint.emit(&self.name, self.router_type))
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
