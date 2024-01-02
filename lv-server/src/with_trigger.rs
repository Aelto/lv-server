use actix_web::HttpResponse;

pub trait WithTrigger {
  fn into_trigger(self) -> &'static str;

  fn trigger(self, res: HttpResponse) -> HttpResponse
  where
    Self: Sized
  {
    super::responses::trigger(res, self.into_trigger())
  }
}

impl WithTrigger for () {
  fn into_trigger(self) -> &'static str {
    ""
  }
}
