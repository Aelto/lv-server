use proc_macro::TokenStream;

pub struct Procedure {
  input: proc_macro2::TokenStream
}

impl Procedure {
  pub fn parse(input: TokenStream) -> Self {
    Self { input: input.into() }
  }

  pub fn emit(self) -> TokenStream {
    let static_variable = Self::emit_static_variable(self);

    proc_macro::TokenStream::from(static_variable)
  }

  fn emit_static_variable(self) -> proc_macro2::TokenStream {
    let handler = proc_macro2::TokenStream::from(self.input);

    quote::quote! {
      LazyLock::new(|| {
        #handler
        let mut hasher = lv_server::deps::blake3::Hasher::new();
        if let Some(last) = file!().rsplit_once("/") {
          hasher.update(last.1.as_bytes());
        }
        hasher.update(&line!().to_ne_bytes());
        let mut url = hasher.finalize().to_string();
        url.truncate(16);

        let full_url = format!("/lvsrv/procs/{url}");
        lv_server::procedures::registry::add_handler_if_missing(url, endpoint);
        full_url
      })
    }
  }
}
