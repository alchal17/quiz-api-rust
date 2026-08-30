use actix_web::web;

use crate::quiz_question_option::routes::handlers::{
    create_quiz_question_option, delete_quiz_question_option, get_all_quiz_question_options,
    get_options_by_question_id, get_quiz_quesiton_optoin_by_id, update_quiz_question_option,
};

mod handlers {
    use actix_web::{
        HttpResponse, Responder, delete, get, post, put,
        web::{Data, Json, Path},
    };

    use crate::{
        common::app_state::AppState,
        quiz_question_option::models::{NewQuizQuestionOptionDto, UpdateQuizQuestionOptionDto},
    };

    #[get("")]
    async fn get_all_quiz_question_options(state: Data<AppState>) -> impl Responder {
        match state
            .quiz_question_option_service
            .get_all_quiz_question_options()
        {
            Ok(quiz_question_options) => HttpResponse::Ok().json(quiz_question_options),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[get("/{id}")]
    async fn get_quiz_quesiton_optoin_by_id(
        state: Data<AppState>,
        path: Path<i32>,
    ) -> impl Responder {
        let id = path.into_inner();

        match state
            .quiz_question_option_service
            .get_quiz_question_option_by_id(id)
        {
            Ok(optional_quiz_quesiton_option_dto) => {
                if let Some(quiz_quesiton_option_dto) = optional_quiz_quesiton_option_dto {
                    HttpResponse::Ok().json(quiz_quesiton_option_dto)
                } else {
                    HttpResponse::NotFound().body(format!(
                        "No quiz quesiton option with id {id} has been found."
                    ))
                }
            }
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[get("by_quiz_id/{id}")]
    async fn get_options_by_question_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let question_id = path.into_inner();

        match state
            .quiz_question_option_service
            .get_quiz_quesiton_options_by_quiz_question_id(question_id)
        {
            Ok(options_dto) => HttpResponse::Ok().json(options_dto),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[post("")]
    async fn create_quiz_question_option(
        state: Data<AppState>,
        json: Json<NewQuizQuestionOptionDto>,
    ) -> impl Responder {
        let new_quiz_question_option_dto = json.into_inner();

        match state
            .quiz_question_option_service
            .create_quiz_question_option(new_quiz_question_option_dto)
        {
            Ok(option_dto) => HttpResponse::Ok().json(option_dto),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[put("/{id}")]
    async fn update_quiz_question_option(
        state: Data<AppState>,
        path: Path<i32>,
        json: Json<UpdateQuizQuestionOptionDto>,
    ) -> impl Responder {
        let update_option_dto = json.into_inner();
        let id = path.into_inner();

        match state
            .quiz_question_option_service
            .update_quiz_question_option(id, update_option_dto)
        {
            Ok(option_dto) => HttpResponse::Ok().json(option_dto),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[delete("/{id}")]
    async fn delete_quiz_question_option(state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let id = path.into_inner();
        match state
            .quiz_question_option_service
            .delete_quiz_question_option(id)
        {
            Ok(_) => HttpResponse::Ok().body(format!(
                "Quiz question option with id {id} deleted successfully."
            )),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

pub fn get_quiz_question_option_routes() -> actix_web::Scope {
    web::scope("/quiz_question_option")
        .service(get_all_quiz_question_options)
        .service(get_quiz_quesiton_optoin_by_id)
        .service(get_options_by_question_id)
        .service(create_quiz_question_option)
        .service(update_quiz_question_option)
        .service(delete_quiz_question_option)
}
