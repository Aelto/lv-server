use maud::Render;

pub trait ExtMaudMarkup {
  /// Offers the ability to append the given [maud::Markup] to Self, resulting
  /// in a new [maud::Markup] with `self` and `other` right after it.
  fn join(self, other: maud::Markup) -> Self;

  /// Turns the current [maud::Markup] into a HTTP response fit for the client.
  fn into_response(self) -> crate::responses::HttpResponse;

  /// Turns the current [maud::Markup] into a HTTP response fit for the client,
  /// while also sending a trigger for the provided event.
  ///
  /// Refer to the [`event!`] macro for events.
  fn into_response_with_event(
    self, event: impl crate::WithTrigger
  ) -> crate::responses::HttpResponse;
}

impl ExtMaudMarkup for maud::Markup {
  fn join(mut self, other: maud::Markup) -> Self {
    other.render_to(&mut self.0);
    self
  }

  fn into_response(self) -> crate::responses::HttpResponse {
    crate::responses::html(self)
  }

  fn into_response_with_event(
    self, event: impl crate::WithTrigger
  ) -> crate::responses::HttpResponse {
    event.trigger(self.into_response())
  }
}
