use proc_macro::TokenStream;

mod parser;

#[proc_macro]
pub fn endpoints(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let (_, model) = parser::Router::parse(&content).unwrap_or_default();
  let output = model.to_string();

  // use the following to debug outputs
  // eprintln!("{output}");

  use std::str::FromStr;
  TokenStream::from_str(&output).unwrap_or_default()
}

mod prelude {
  pub use nom::bytes::complete::{tag, take_until1, take_while};
  pub use nom::error::ParseError;
  pub use nom::multi::many0;
  pub use nom::IResult;

  pub fn trim(i: &str) -> IResult<&str, &str> {
    take_while(|c| c == ' ' || c == '\n' || c == '\r')(i)
  }
}
