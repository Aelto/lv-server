use crate::prelude::*;

mod field;
pub use field::Event;

#[derive(Debug, Clone, Default)]
pub struct Events {
  name: String,
  variants: Vec<Event>
}

impl Events {
  pub fn parse(i: &str) -> IResult<&str, Self> {
    let (i, _) = trim(i)?;
    let (i, name) = take_until1("{")(i)?;
    let (i, _) = tag("{")(i)?;
    let (i, variants) = many0(Event::parse)(i)?;
    let (i, _) = nom::bytes::complete::take_until("}")(i)?;

    Ok((
      i,
      Self {
        variants,
        name: name.trim().to_owned()
      }
    ))
  }

  /// Emits a router that combines all of the endpoint routers
  fn emit_enum(&self) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", self.name);
    let variants: Vec<proc_macro2::TokenStream> = self
      .variants
      .iter()
      .map(|v| {
        let variant = quote::format_ident!("{}", v.name);

        quote::quote!(#variant)
      })
      .collect();

    quote::quote!(
      pub enum #name {
        #(#variants),*
      }
    )
  }

  fn emit_with_trigger_impl(&self) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", self.name);
    let into_trigger_variants: Vec<proc_macro2::TokenStream> = self
      .variants
      .iter()
      .map(|v| {
        let variant = quote::format_ident!("{}", v.name);
        let event_str = format!("{}_{}", name, v.name);

        quote::quote!(
          Self::#variant => #event_str
        )
      })
      .collect();

    quote::quote!(
      impl lv_server::WithTrigger for #name {
        fn into_trigger(self) -> &'static str {
          match self {
            #(#into_trigger_variants),*
          }
        }
      }
    )
  }

  fn emit_maud_render_impl(&self) -> proc_macro2::TokenStream {
    let name = quote::format_ident!("{}", self.name);
    let render_variants: Vec<proc_macro2::TokenStream> = self
      .variants
      .iter()
      .map(|v| {
        let variant = quote::format_ident!("{}", v.name);
        let event_str = format!("{}_{} {}", name, v.name, v.modifier);

        quote::quote!(
          Self::#variant => buffer.push_str(#event_str)
        )
      })
      .collect();

    quote::quote!(
      impl maud::Render for #name {
        fn render_to(&self, buffer: &mut String) {
          match self {
            #(#render_variants),*
          };
        }
      }
    )
  }
}

impl std::fmt::Display for Events {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let enum_def = self.emit_enum();
    let enum_with_trigger_impl = self.emit_with_trigger_impl();
    let enum_maud_render_impl = self.emit_maud_render_impl();

    let output = quote::quote! {
      #enum_def
      #enum_with_trigger_impl
      #enum_maud_render_impl
    };

    write!(f, "{output}")
  }
}
