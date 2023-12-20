use crate::prelude::*;

pub struct ViewLibrary {
  library_title: String
}

// impl lv_server::View for ViewLibrary {}

// impl WithRouter for ViewLibrary {
//   fn router(cfg: &mut actix_web::web::ServiceConfig) {
//     cfg.route("/library/{title}", get().to(index));

//     async fn index(path: Path<String>) -> HttpResponse {
//       lv_server::responses::html(
//         ViewLibrary {
//           library_title: path.into_inner()
//         }
//         .render()
//         .await
//       )
//     }
//   }
// }
