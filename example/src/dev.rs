//! code meant to speed-up dev-time, often using hardcoded temporary solutions

use crate::prelude::*;

pub fn signed_user() -> Author {
  Author::find_all()
    .unwrap()
    .into_iter()
    .find(|u| u.handle == "SignedUser")
    .unwrap()
}
