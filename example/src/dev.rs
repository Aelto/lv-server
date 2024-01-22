//! code meant to speed-up dev-time, often using hardcoded temporary solutions

use crate::prelude::*;

pub async fn signed_user() -> Author {
  Author::find_all()
    .await
    .unwrap()
    .into_iter()
    .find(|u| u.handle == "SignedUser")
    .unwrap()
}
