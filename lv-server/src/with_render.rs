/// A wrapping implementation of [maud::Render] but with async support
///
/// ```rs
/// async fn render(&self) -> maud::Markup {
///   html!()
/// }
/// ```
#[async_trait::async_trait]
pub trait WithRender {
  async fn render(&self) -> maud::Markup;
}
