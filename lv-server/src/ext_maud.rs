use maud::Render;
use crate::responses::alert;

pub trait ExtMaudMarkup {
  /// Offers the ability to append the given [maud::Markup] to Self, resulting
  /// in a new [maud::Markup] with `self` and `other` right after it.
  ///
  /// ```
  /// use lv_server::ExtMaudMarkup;
  /// use maud::html;
  ///
  /// let first = html!(h1 {"Hello world!"});
  /// let second = html!(p {"Lorem ipsum"});
  /// let joined = first.join(second).into_string();
  ///
  /// let first = html!(h1 {"Hello world!"});
  /// let second = html!(p {"Lorem ipsum"});
  /// let manual = html!((first)(second)).into_string();
  ///
  /// assert_eq!(joined, manual);
  /// ```
  fn join(self, other: maud::Markup) -> Self;

  /// Offers the ability to append an alert to Self, resulting
  /// in a new [maud::Markup] with `self` and alert right after it.
  ///
  /// refer to [alert] for more information on setting up alerts.
  ///
  /// ```rs
  /// let view: maud::Markup = render_view();
  ///
  /// view
  ///   .join_alert("success", "The operation was successful!")
  ///   .into_response()
  /// ```
  fn join_alert(self, class: &str, message: &impl maud::Render) -> Self;

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

  fn join_alert(mut self, class: &str, message: &impl maud::Render) -> Self {
    alert(class, message).render_to(&mut self.0);
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

#[test]
fn test_join_alert() {
  let view = maud::html!(h1 {"Hello World!"});
  let alert = crate::responses::alert("success", &"the operation was a success");

  let manual = maud::html!((view)(alert));
  let joined = view.join(alert);

  assert_eq!(manual.into_string(), joined.into_string());
}
