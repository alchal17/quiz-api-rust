use actix_web::web;

use crate::quizzes::routes::handlers::{
    create_new_quiz, delete_quiz, get_all_quizzes, get_quiz_by_id, update_quiz,
};

mod handlers {
    use actix_web::{
        HttpResponse, Responder, delete, get, post, put,
        web::{Data, Json, Path},
    };

    use crate::{
        common::app_state::AppState,
        quizzes::models::{NewQuizDto, UpdateQuizDto},
    };

    #[get("")]
    async fn get_all_quizzes(app_state: Data<AppState>) -> impl Responder {
        match app_state.quiz_serice.get_all().await {
            Ok(quiz_dtos) => HttpResponse::Ok().json(quiz_dtos),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[get("/{id}")]
    async fn get_quiz_by_id(app_state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let id = path.into_inner();

        match app_state.quiz_serice.get_by_id(id).await {
            Ok(optional_user) => match optional_user {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().body(format!("User with id {} not found.", id)),
            },
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[post("")]
    async fn create_new_quiz(app_state: Data<AppState>, json: Json<NewQuizDto>) -> impl Responder {
        let new_quiz_dto = json.into_inner();
        match app_state.quiz_serice.create_quiz(new_quiz_dto).await {
            Ok(quiz_dto) => HttpResponse::Created().json(quiz_dto),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[put("/{id}")]
    async fn update_quiz(
        app_state: Data<AppState>,
        path: Path<i32>,
        json: Json<UpdateQuizDto>,
    ) -> impl Responder {
        let id = path.into_inner();
        let updating_quiz = json.into_inner();

        match app_state.quiz_serice.update_quiz(id, updating_quiz).await {
            Ok(quiz) => HttpResponse::Ok().json(quiz),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
    #[delete("/{id}")]
    async fn delete_quiz(app_state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let id = path.into_inner();
        match app_state.quiz_serice.delete_quiz(id).await {
            Ok(()) => HttpResponse::Ok().body(format!("Quiz with id {} deleted.", id)),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

pub fn quiz_routes() -> actix_web::Scope {
    web::scope("/quiz")
        .service(get_all_quizzes)
        .service(get_quiz_by_id)
        .service(create_new_quiz)
        .service(update_quiz)
        .service(delete_quiz)
}
