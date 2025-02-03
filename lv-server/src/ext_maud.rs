use maud::Render;

pub trait ExtMaudMarkup {
  /// Offers the ability to append the given [maud::Markup] to Self, resulting
  /// in a new [maud::Markup] with `self` and `other` right after it.
  fn join(self, other: maud::Markup) -> Self;

  /// Turns the current [maud::Markup] into a HTTP response fit for the client.
  fn into_response(self) -> crate::responses::HttpResponse;
}

impl ExtMaudMarkup for maud::Markup {
  fn join(mut self, other: maud::Markup) -> Self {
    other.render_to(&mut self.0);
    self
  }

  fn into_response(self) -> crate::responses::HttpResponse {
    crate::responses::html(self)
  }
}
