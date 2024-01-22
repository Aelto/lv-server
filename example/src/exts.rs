pub mod option {
  use crate::prelude::*;

  pub trait OptionExt<V> {
    fn conflict_if_some(self, message: &'static str) -> AppResult<()>;
    fn conflict_if_none(self, message: &'static str) -> AppResult<V>;
    fn not_found_if_none(self, message: &'static str) -> AppResult<V>;
    fn internal_error_if_none(self, message: &'static str) -> AppResult<V>;
    fn unauthorized_if_none(self, message: &'static str) -> AppResult<V>;
    fn empty_if_none(self) -> AppResult<V>;
    fn json_if_none(self, json: impl serde::Serialize) -> AppResult<V>;

    /// Return early with `other` if `Self` is `None`.
    fn else_if_none<T>(self, other: T) -> Result<V, T>;
    fn default_if_none<T>(self) -> Result<V, T>
    where
      T: Default;

    /// Renders the `Option` into a `maud::Markup`, resulting in an empty markup
    /// if it is `None`
    fn render(self) -> maud::Markup
    where
      V: maud::Render;
  }

  impl<V> OptionExt<V> for Option<V> {
    fn conflict_if_some(self, message: &'static str) -> AppResult<()> {
      match self {
        Some(_) => Err(AppError::Conflict(message)),
        None => Ok(())
      }
    }

    fn conflict_if_none(self, message: &'static str) -> AppResult<V> {
      match self {
        Some(v) => Ok(v),
        None => Err(AppError::Conflict(message))
      }
    }

    fn not_found_if_none(self, message: &'static str) -> AppResult<V> {
      match self {
        Some(v) => Ok(v),
        None => Err(AppError::NotFound(message))
      }
    }

    fn internal_error_if_none(self, message: &'static str) -> AppResult<V> {
      match self {
        Some(v) => Ok(v),
        None => Err(AppError::InternalServerError(message))
      }
    }

    fn empty_if_none(self) -> AppResult<V> {
      match self {
        Some(v) => Ok(v),
        None => Err(AppError::Empty)
      }
    }

    fn json_if_none(self, json: impl serde::Serialize) -> AppResult<V> {
      match self {
        Some(v) => Ok(v),
        None => Err(AppError::Message(serde_json::to_value(json)?))
      }
    }

    fn else_if_none<T>(self, other: T) -> Result<V, T> {
      match self {
        Some(v) => Ok(v),
        None => Err(other)
      }
    }

    fn default_if_none<T>(self) -> Result<V, T>
    where
      T: Default
    {
      match self {
        Some(v) => Ok(v),
        None => Err(T::default())
      }
    }

    fn unauthorized_if_none(self, message: &'static str) -> AppResult<V> {
      match self {
        Some(v) => Ok(v),
        None => Err(AppError::Unauthorized(message))
      }
    }

    fn render(self) -> maud::Markup
    where
      V: maud::Render
    {
      match self {
        Some(v) => v.render(),
        None => maud::html!()
      }
    }
  }
}
pub mod result {
  use crate::prelude::*;

  pub trait ResultExt<V> {
    fn internal_error_if_err(self, message: &'static str) -> AppResult<V>;

    /// Use that function if there is a place where you think it is okay to have
    /// a None or an Err and you just want to early exit the function with an
    /// error that will render to empty HTML.
    fn empty_if_err(self) -> AppResult<V>;

    /// Renders the `Result` into a `maud::Markup`
    fn render(self) -> maud::Markup
    where
      V: maud::Render;

    /// Renders the Err part of the result, keeping the Ok part unchanged
    fn render_err(self) -> Result<V, maud::Markup>;
  }

  impl<V> ResultExt<V> for AppResult<V> {
    fn internal_error_if_err(self, message: &'static str) -> AppResult<V> {
      match self {
        Ok(v) => Ok(v),
        Err(_) => Err(AppError::InternalServerError(message))
      }
    }

    fn empty_if_err(self) -> AppResult<V> {
      match self {
        Ok(v) => Ok(v),
        Err(_) => Err(AppError::Empty)
      }
    }

    fn render(self) -> maud::Markup
    where
      V: maud::Render
    {
      match self {
        Ok(v) => maud::Render::render(&v),
        Err(e) => maud::Render::render(&e)
      }
    }

    fn render_err(self) -> Result<V, maud::Markup> {
      self.map_err(|e| maud::Render::render(&e))
    }
  }

  pub trait ResultTupleExt<L, R> {
    /// Extracts out the right part of the result, yielding a new result with the
    /// only the left part in it and an option with the right part that was just
    /// extracted.
    ///
    /// The right part is an option as it defaults to a [None] when the result is
    /// an [Err].
    fn extract_right(self) -> (AppResult<L>, Option<R>);

    /// Extracts out the left part of the result, yielding a new result with the
    /// only the right part in it and an option with the left part that was just
    /// extracted.
    ///
    /// The left part is an option as it defaults to a [None] when the result is
    /// an [Err].
    fn extract_left(self) -> (Option<L>, AppResult<R>);
  }
  impl<L, R> ResultTupleExt<L, R> for AppResult<(L, R)> {
    fn extract_right(self) -> (AppResult<L>, Option<R>) {
      match self {
        Ok((l, r)) => (Ok(l), Some(r)),
        Err(e) => (Err(e), None)
      }
    }

    fn extract_left(self) -> (Option<L>, AppResult<R>) {
      match self {
        Ok((l, r)) => (Some(l), Ok(r)),
        Err(e) => (None, Err(e))
      }
    }
  }
}
pub mod vec {
  use crate::prelude::*;

  pub trait VecExt<R> {
    /// Maps the vector and render each item into a single `Markup`
    fn map_render<F>(&self, f: F) -> maud::Markup
    where
      F: Fn(&R) -> maud::Markup;

    /// Combine all of the items in the vec into a single joined markup list
    fn join_render(self) -> maud::Markup
    where
      R: maud::Render;

    /// Maps to an empty Error for easy early-return if the Vec is empty
    fn empty_if_empty(&self) -> AppResult<&Self>
    where
      Self: Sized;
  }

  impl<R> VecExt<R> for Vec<R> {
    fn map_render<F>(&self, f: F) -> maud::Markup
    where
      F: Fn(&R) -> maud::Markup
    {
      maud::html!(
        @for item in self {
          (f(&item))
        }
      )
    }

    fn join_render(self) -> maud::Markup
    where
      R: maud::Render
    {
      maud::html!(
        @for item in self {
          (item)
        }
      )
    }

    fn empty_if_empty(&self) -> AppResult<&Self>
    where
      Self: Sized
    {
      match self.is_empty() {
        false => Ok(self),
        true => Err(AppError::Empty)
      }
    }
  }
}

pub mod foreign {
  use crate::prelude::*;

  pub trait ForeignExt {
    fn fk(&self) -> &Id;
  }

  impl<T> ForeignExt for ForeignKey<T, Id> {
    fn fk(&self) -> &FlexibleId {
      match self.key() {
        Some(k) => k,
        None => &FlexibleId::Empty
      }
    }
  }
}
