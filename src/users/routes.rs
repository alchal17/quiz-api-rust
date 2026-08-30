use actix_web::web;

mod handlers {
    use actix_web::{
        HttpResponse, Responder, delete, get, post, put, web::Data, web::Json, web::Path,
    };

    use crate::{
        common::app_state::AppState,
        users::models::{NewQuizUserDto, UpdateQuizUserDto},
    };
    #[get("")]
    async fn get_all_users(state: Data<AppState>) -> impl Responder {
        match state.user_service.get_all_users().await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[get("/{id}")]
    async fn get_by_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let id = path.into_inner();
        match state.user_service.get_user_by_id(id).await {
            Ok(option_user) => match option_user {
                Some(user) => HttpResponse::Ok().json(user),
                None => {
                    HttpResponse::NotFound().body(format!("User with id {} has not been found", id))
                }
            },
            Err(error) => HttpResponse::InternalServerError().body(error),
        }
    }
    #[get("/username/{username}")]
    async fn get_by_username(state: Data<AppState>, path: Path<String>) -> impl Responder {
        let username = path.into_inner();
        match state.user_service.find_by_username(&username).await {
            Ok(option_user) => match option_user {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound()
                    .body(format!("User with username {} not found.", username)),
            },
            Err(error) => HttpResponse::InternalServerError().body(error),
        }
    }
    #[get("/email/{email}")]
    async fn get_by_email(state: Data<AppState>, path: Path<String>) -> impl Responder {
        let email = path.into_inner();
        match state.user_service.find_by_email(&email).await {
            Ok(option_user) => match option_user {
                Some(user) => HttpResponse::Ok().json(user),
                None => {
                    HttpResponse::NotFound().body(format!("User with email {} not found.", email))
                }
            },
            Err(error) => HttpResponse::InternalServerError().body(error),
        }
    }
    #[post("")]
    async fn create_new_user(state: Data<AppState>, json: Json<NewQuizUserDto>) -> impl Responder {
        let user_data = json.into_inner();
        match state.user_service.create_new_user(user_data).await {
            Ok(new_user) => HttpResponse::Ok().json(new_user),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[put("/{id}")]
    async fn update_user(
        state: Data<AppState>,
        path: Path<i32>,
        json: Json<UpdateQuizUserDto>,
    ) -> impl Responder {
        let user_id = path.into_inner();
        let update_quiz_user_dto = json.into_inner();

        match state
            .user_service
            .update(user_id, update_quiz_user_dto)
            .await
        {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[delete("/{id}")]
    async fn delete_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let user_id = path.into_inner();
        match state.user_service.delete_user(user_id).await {
            Ok(_) => HttpResponse::Ok().body(format!("User with id {} has been deleted", user_id)),
            Err(error) => HttpResponse::BadRequest().body(format!(
                "Error deleting user with id {}: {}",
                user_id, error
            )),
        }
    }
}

pub fn user_routes() -> actix_web::Scope {
    web::scope("/quiz_user")
        .service(handlers::get_all_users)
        .service(handlers::create_new_user)
        .service(handlers::get_by_username)
        .service(handlers::get_by_email)
        .service(handlers::get_by_id)
        .service(handlers::update_user)
        .service(handlers::delete_user)
}
