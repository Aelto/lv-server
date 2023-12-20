use crate::WithRouter;

/// Represents the core of the page, which can be composed of as many
/// [Fragment](super::Fragment) as needed to render completely.
pub trait View: WithRouter {}
