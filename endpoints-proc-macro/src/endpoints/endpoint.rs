use nom::{
  bytes::complete::{is_not, take_until},
  character::complete::char,
  sequence::delimited
};
use quote::format_ident;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Endpoint {
  pub name: String,
  verb: String,
  route: String,
  params: Vec<String>,
  extends: Vec<String>,
  service_options: Vec<super::ServiceOption>
}

impl Endpoint {
  pub fn parse(i: &str) -> IResult<&str, Self> {
    let (i, _) = trim(i)?;
    let (i, name) = take_until1(" ")(i)?;
    let (i, extends) = many0(Self::parse_extend)(i)?;
    let (i, options) = take_until1("=>")(i)?;
    let (i, _) = trim(i)?;
    let (i, _) = tag("=>")(i)?;
    let (i, _) = trim(i)?;
    let (i, verb) = take_until1(" ")(i)?;
    let (i, _) = trim(i)?;
    let (i, _) = tag("\"")(i)?;
    let (i, route) = take_until1("\"")(i)?;
    let (i, _) = tag("\"")(i)?;

    let (_, options) = many0(super::ServiceOption::parse)(options)?;
    let (_, params) = many0(Self::parse_param)(route)?;

    Ok((
      i,
      Self {
        name: name.trim().to_owned(),
        verb: verb.trim().to_owned(),
        route: route.to_owned(),
        params,
        extends,
        service_options: options
      }
    ))
  }

  fn parse_extend(i: &str) -> IResult<&str, String> {
    let (i, _) = trim(i)?;
    let (i, _) = tag("extend")(i)?;
    let (i, _) = take_until("(")(i)?;
    let (i, ty) = delimited(char('('), is_not(")"), char(')'))(i)?;

    Ok((i, ty.to_owned()))
  }

  fn parse_param(route: &str) -> IResult<&str, String> {
    let (i, _) = nom::bytes::complete::take_until("{")(route)?;
    let (i, _) = tag("{")(i)?;
    let (i, param) = take_until1("}")(i)?;
    let (i, _) = tag("}")(i)?;

    Ok((i, param.to_owned()))
  }

  pub fn emit(
    &self, router_name: &str, router_type: super::RouterType
  ) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", self.name);
    let route = &self.route;

    let url_fn = self.emit_url_fn(router_name);
    let route_fn = self.emit_route_fn(router_name, router_type);
    let router = self.emit_router();

    let output = quote::quote!(
      pub mod #name {
        use super::*;
        pub const URL: &'static str = #route;
        #router

        #url_fn
        #route_fn
      }
    );

    output
  }

  fn emit_url_fn(&self, router_name: &str) -> proc_macro2::TokenStream {
    let router_name = format_ident!("{}", router_name);
    let route = &self.route;
    let params = self.params.iter().map(|p| format_ident!("{}", p));

    quote::quote!(
      pub fn url(#(#params : &str),*) -> String {
        use lv_server::Fragment;
        use lv_server::View;

        super::super::#router_name::url(&format!(#route))
      }
    )
  }

  fn emit_route_fn(
    &self, router_name: &str, router_type: super::RouterType
  ) -> proc_macro2::TokenStream {
    let router_name = format_ident!("{}", router_name);
    let verb = format_ident!("{}", self.verb.to_lowercase());
    let verb_upper = format_ident!("{}", self.verb.to_uppercase());
    let extends: Vec<proc_macro2::TokenStream> = self
      .extends
      .iter()
      .map(|ex| {
        let ex = format_ident!("{}", ex);
        quote::quote!(
          let route = super::super::#router_name::#ex(route);
        )
      })
      .collect();
    let service_options: Vec<proc_macro2::TokenStream> =
      self.service_options.iter().map(|s| s.emit()).collect();

    let route_fn = match router_type {
      crate::endpoints::RouterType::Fragment => quote::quote!(
        let route = super::super::#router_name::fragment_route(
          cfg,
          URL,
          lv_server::csrf::csrf_protection(
            actix_web::web::#verb(),
            actix_web::http::Method::#verb_upper
          ).to(handler)
          #(#service_options)*
        );
        #(#extends)*
      ),
      crate::endpoints::RouterType::View => quote::quote!(
        let route = super::super::#router_name::view_route(
          cfg,
          URL,
          lv_server::csrf::csrf_protection(
            actix_web::web::#verb(),
            actix_web::http::Method::#verb_upper
          ).to(handler)
          #(#service_options)*
        );
        #(#extends)*
      )
    };

    quote::quote!(
      pub fn route<F, ARGS>(cfg: &mut actix_web::web::ServiceConfig, handler: F)
      where
        F: actix_web::Handler<ARGS>,
        ARGS: actix_web::FromRequest + 'static,
        F::Output: actix_web::Responder + 'static
      {
        use lv_server::Fragment;
        use lv_server::View;
        #route_fn
      }
    )
  }

  fn emit_router(&self) -> proc_macro2::TokenStream {
    quote::quote!(
      pub struct Router;
      impl lv_server::WithRouter for Router {
        fn router(cfg: &mut actix_web::web::ServiceConfig) {
          route(cfg, Router::endpoint);
        }
      }
    )
  }
}
