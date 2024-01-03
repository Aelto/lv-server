use std::fmt::Display;

use crate::prelude::*;

use super::Endpoint;

#[derive(Debug, Clone, Default)]
pub struct Router {
  name: String,
  endpoints: Vec<Endpoint>
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
        name: name.trim().to_owned()
      }
    ))
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

    let output = quote::quote! {
      pub mod api {
        #(#endpoint_mods)*
      }
    };

    write!(f, "{output}")
  }
}
