use crate::WithRouter;

pub trait Fragment: WithRouter + maud::Render {}
