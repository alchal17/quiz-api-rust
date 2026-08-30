use actix_web::web;

use crate::images::routes::handlers::get_image;

mod handlers {
    use crate::common::constants::IMAGE_DIR;
    use crate::images::image_manager::read_image;
    use actix_web::{HttpResponse, Responder, get, web::Path};

    #[get("/{image_name}")]
    async fn get_image(path: Path<String>) -> impl Responder {
        let image_name = path.into_inner();

        let result = actix_web::web::block(move || read_image(&image_name, IMAGE_DIR)).await;

        match result {
            Ok(Ok((bytes, content_type))) => {
                HttpResponse::Ok().content_type(content_type).body(bytes)
            }
            Ok(Err(e)) => {
                if e.contains("not found") {
                    HttpResponse::NotFound().body(e)
                } else if e.contains("Invalid filename") {
                    HttpResponse::BadRequest().body(e)
                } else {
                    HttpResponse::InternalServerError().body(e)
                }
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("Server error: {}", e)),
        }
    }
}

pub fn image_routes() -> actix_web::Scope {
    web::scope("/images").service(get_image)
}
