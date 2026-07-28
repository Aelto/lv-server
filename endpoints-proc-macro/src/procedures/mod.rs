use std::{collections::HashMap, sync::Mutex};

use proc_macro::TokenStream;

pub static REGISTRY: std::sync::LazyLock<Mutex<HashMap<String, String>>> =
  std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
