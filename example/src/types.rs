pub use flexible_id::FlexibleId;
pub use flexible_id::FlexibleId as Id;
pub use id_representation::IdRepresentation;
pub use relation::Relation;
pub use relation::RelationOptionExt;

mod flexible_id {
  use serde::Deserialize;
  use serde::Serialize;
  use surrealdb::sql::thing;
  use surrealdb::sql::Thing;

  use super::IdRepresentation;

  /// This type doesn't implement Serialize on purpose, if you wish to serialize it
  /// then transform it into an [Id] by doing `Id::from(self)` which implements
  /// Serialize.
  ///
  /// Same for the other utility types like AsRef, Deref, etc... [FlexibleId] is
  /// only meant to be used to __deserialize__ IDs that can potentially be in two
  /// different forms:
  /// - Thing
  /// - String
  ///
  /// Note that this type may fail to deserialize if use in a Actix [Path]
  #[derive(Debug, PartialEq, Clone, Deserialize)]
  #[serde(untagged)]
  pub enum FlexibleId {
    Thing(Thing),
    String(String),
    Empty
  }

  impl FlexibleId {
    pub fn is_empty(&self) -> bool {
      match self {
        Self::Empty => true,
        _ => false
      }
    }

    pub fn new_thing(tb: String, id: impl Into<surrealdb::sql::Id>) -> Self {
      Self::Thing(Thing { tb, id: id.into() })
    }

    pub fn into_thing(self) -> Result<Thing, surrealdb::error::Db> {
      match self {
        FlexibleId::Thing(t) => Ok(t),
        FlexibleId::String(s) => thing(&s),
        FlexibleId::Empty => Err(surrealdb::error::Db::IdInvalid {
          value: "Empty".to_owned()
        })
      }
    }

    pub fn to_thing(&self) -> Result<Thing, surrealdb::error::Db> {
      match self {
        FlexibleId::Thing(t) => Ok(t.clone()),
        FlexibleId::String(s) => thing(s),
        FlexibleId::Empty => Err(surrealdb::error::Db::IdInvalid {
          value: "Empty".to_owned()
        })
      }
    }

    /// Gets the (tb, id) parts of this id. In some cases where the parsing fails
    /// the `id` part may be empty.
    pub fn parts(&self) -> (&str, &str) {
      match self {
        FlexibleId::Thing(s) => (
          &s.tb,
          match &s.id {
            surrealdb::sql::Id::String(id) => id.as_str(),
            _ => ""
          }
        ),
        FlexibleId::String(s) => s.split_once(':').unwrap_or((&s, "")),
        FlexibleId::Empty => ("", "")
      }
    }

    /// ```
    /// let table = "table".to_owned();
    /// let id = "some-complex/uuid";
    ///
    /// let tbid = FlexibleId::new_thing(table, id);
    ///
    /// assert_eq!(tbid.id(), id);
    /// ```
    pub fn id(&self) -> &str {
      self.parts().1
    }

    pub fn tb(&self) -> &str {
      self.parts().0
    }
  }

  impl Serialize for FlexibleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer
    {
      match self {
        FlexibleId::Thing(thing) => thing.serialize(serializer),
        FlexibleId::String(s) => match surrealdb::sql::thing(s) {
          Ok(t) => t.serialize(serializer),
          Err(_) => Err(serde::ser::Error::custom("thing(string) error"))
        },
        FlexibleId::Empty => Err(serde::ser::Error::custom("thing(empty) error"))
      }
    }
  }

  impl std::fmt::Display for FlexibleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        FlexibleId::Thing(t) => t.fmt(f),
        FlexibleId::String(s) => s.fmt(f),
        FlexibleId::Empty => Ok(())
      }
    }
  }

  impl From<String> for FlexibleId {
    fn from(value: String) -> Self {
      Self::String(value)
    }
  }

  impl From<&str> for FlexibleId {
    fn from(value: &str) -> Self {
      value.to_owned().into()
    }
  }

  impl Default for FlexibleId {
    fn default() -> Self {
      Self::Empty
    }
  }

  impl From<IdRepresentation> for FlexibleId {
    fn from(value: IdRepresentation) -> Self {
      Self::from(value.0)
    }
  }

  impl maud::Render for FlexibleId {
    fn render(&self) -> maud::Markup {
      self.to_string().render()
    }
  }
}

mod id_representation {
  use serde::Deserialize;
  use serde::Serialize;

  use super::FlexibleId;

  /// This type is to be used if the frontend requires ID for its logic, if you were
  /// to use a simple [Id] in such a case then you would encounter problems as they
  /// are deserialized into the [Thing] form whereas [IdRepresentation] doesn't.
  #[derive(Debug, PartialEq, Clone, Default)]
  pub struct IdRepresentation(pub(crate) String);

  impl<'de> Deserialize<'de> for IdRepresentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>
    {
      Ok(Self::from(String::deserialize(deserializer)?))
    }
  }

  impl Serialize for IdRepresentation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer
    {
      self.0.serialize(serializer)
    }
  }

  impl std::fmt::Display for IdRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.0)
    }
  }

  impl From<String> for IdRepresentation {
    fn from(value: String) -> Self {
      Self(value)
    }
  }

  impl From<&str> for IdRepresentation {
    fn from(value: &str) -> Self {
      value.to_owned().into()
    }
  }

  impl From<FlexibleId> for IdRepresentation {
    fn from(value: FlexibleId) -> Self {
      match value {
        FlexibleId::Thing(t) => Self(t.to_string()),
        FlexibleId::String(s) => Self(s),
        FlexibleId::Empty => Self::default()
      }
    }
  }
}

mod relation {
  use serde::Deserialize;

  /// A custom type used to extract relations out of query results. To use it you
  /// must alias the relation with `as relation` and take the result wrapped by
  /// this type.
  #[derive(Debug, Deserialize)]
  pub struct Relation<T> {
    /// when doing queries with edges there is no way to select 1 end of the edge.
    /// It's always a vector of results hence the `Vec<T>`, we then offer methods
    /// to get the first element or multiple elements depending on the need
    relation: Vec<T>
  }

  impl<T> Relation<T> {
    pub fn first(mut self) -> Option<T> {
      self.relation.pop()
    }

    /// Returns all the elements from the queried relation/edge
    pub fn all(self) -> Vec<T> {
      self.relation
    }
  }

  pub trait RelationOptionExt<T> {
    /// If the Option is Some then get the first element from the list of queried
    /// relations/edges.
    fn first(self) -> Option<T>;

    /// If the Option is Some then get the list of queried relations/edges,
    /// otherwise return an empty vec
    fn all(self) -> Vec<T>;
  }
  impl<T> RelationOptionExt<T> for Option<Relation<T>> {
    fn first(self) -> Option<T> {
      self.and_then(|r| r.first())
    }

    fn all(self) -> Vec<T> {
      self.map(|r| r.all()).unwrap_or_default()
    }
  }
}
