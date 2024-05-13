use nom::{
  branch::alt,
  bytes::complete::{is_not, take_until},
  character::complete::char,
  sequence::delimited
};
use quote::format_ident;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum ServiceOption {
  Wrap(String),
  WrapFromFn(String)
}

impl ServiceOption {
  pub fn parse(i: &str) -> IResult<&str, Self> {
    let (i, _) = trim(i)?;

    alt((Self::parse_wrap_from_fn, Self::parse_wrap))(i)
  }

  fn parse_wrap(i: &str) -> IResult<&str, Self> {
    let (i, _) = tag("wrap")(i)?;
    let (i, _) = take_until("(")(i)?;
    let (i, ty) = delimited(char('('), is_not(")"), char(')'))(i)?;

    Ok((i, Self::Wrap(ty.to_owned())))
  }

  fn parse_wrap_from_fn(i: &str) -> IResult<&str, Self> {
    let (i, _) = tag("wrap_from_fn")(i)?;
    let (i, _) = take_until("(")(i)?;
    let (i, ty) = delimited(char('('), is_not(")"), char(')'))(i)?;

    Ok((i, Self::WrapFromFn(ty.to_owned())))
  }

  pub fn emit(&self) -> proc_macro2::TokenStream {
    match self {
      ServiceOption::Wrap(s) => {
        let ty = format_ident!("{}", s);
        quote::quote!(
          .wrap(#ty)
        )
      }
      ServiceOption::WrapFromFn(s) => {
        let ty = format_ident!("{}", s);
        quote::quote!(
          .wrap(from_fn(#ty))
        )
      }
    }
  }
}
