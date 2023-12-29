use crate::{WithRouter, WithTrigger};

pub trait Fragment<Events>: WithRouter
where
  Events: WithTrigger
{
}
