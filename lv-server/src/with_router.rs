pub trait WithRouter {
  /// The place where the type defines all of the endpoints it depends on
  fn router(cfg: &mut actix_web::web::ServiceConfig);
}

// Blanket implementations to group WithRouter implementators together in tuples

impl WithRouter for () {
  fn router(_: &mut actix_web::web::ServiceConfig) {}
}

impl<R1, R2> WithRouter for (R1, R2)
where
  R1: WithRouter,
  R2: WithRouter
{
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .configure(<R1 as WithRouter>::router)
      .configure(<R2 as WithRouter>::router);
  }
}

impl<R1, R2, R3> WithRouter for (R1, R2, R3)
where
  R1: WithRouter,
  R2: WithRouter,
  R3: WithRouter
{
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .configure(<R1 as WithRouter>::router)
      .configure(<R2 as WithRouter>::router)
      .configure(<R3 as WithRouter>::router);
  }
}

impl<R1, R2, R3, R4> WithRouter for (R1, R2, R3, R4)
where
  R1: WithRouter,
  R2: WithRouter,
  R3: WithRouter,
  R4: WithRouter
{
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .configure(<R1 as WithRouter>::router)
      .configure(<R2 as WithRouter>::router)
      .configure(<R3 as WithRouter>::router)
      .configure(<R4 as WithRouter>::router);
  }
}
