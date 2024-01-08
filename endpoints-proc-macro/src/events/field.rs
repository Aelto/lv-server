use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Event {
  pub name: String,
  pub modifier: String
}

impl Event {
  pub fn parse(i: &str) -> IResult<&str, Self> {
    let (i, _) = trim(i)?;
    let (i, name) = take_until1(" ")(i)?;
    let (i, _) = trim(i)?;
    let (i, _) = tag("\"")(i)?;
    let (i, modifier) = take_until1("\"")(i)?;
    let (i, _) = tag("\"")(i)?;

    Ok((
      i,
      Self {
        name: name.trim().to_owned(),
        modifier: modifier.trim().to_owned()
      }
    ))
  }
}
