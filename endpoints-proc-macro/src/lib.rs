use proc_macro::TokenStream;

mod endpoints;

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

mod prelude {

  pub use nom::bytes::complete::{tag, take_until1, take_while, take_while1};
  pub use nom::error::ParseError;
  pub use nom::multi::many0;
  pub use nom::IResult;

  pub fn trim(i: &str) -> IResult<&str, &str> {
    take_while(|c| c == ' ' || c == '\n' || c == '\r')(i)
  }
}
