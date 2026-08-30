use actix_web::{Scope, web::scope};

use crate::quiz_questions::routes::handlers::{
    create_quiz_question, delete_quiz_question, find_quiz_question_by_id, get_all_quiz_quesitons,
    update_quiz_question,
};

mod handlers {
    use crate::{
        common::app_state::AppState,
        quiz_questions::models::{NewQuizQuestionDto, UpdateQuizQuestionDto},
    };
    use actix_web::{
        HttpResponse, Responder, delete, get, post, put,
        web::{Data, Json, Path},
    };

    #[get("")]
    async fn get_all_quiz_quesitons(app_state: Data<AppState>) -> impl Responder {
        match app_state.quiz_question_service.get_all().await {
            Ok(quiz_questions_dto) => HttpResponse::Ok().json(quiz_questions_dto),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[get("/{id}")]
    async fn find_quiz_question_by_id(
        app_state: Data<AppState>,
        path: Path<i32>,
    ) -> impl Responder {
        let id = path.into_inner();

        match app_state.quiz_question_service.get_by_id(id).await {
            Ok(quiz_question_optional) => match quiz_question_optional {
                Some(quiz_question) => HttpResponse::Ok().json(quiz_question),
                None => HttpResponse::NotFound()
                    .body(format!("Quiz question with id {id} has not been found")),
            },
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[post("")]
    async fn create_quiz_question(
        app_state: Data<AppState>,
        json: Json<NewQuizQuestionDto>,
    ) -> impl Responder {
        let new_quiz_question_dto = json.into_inner();

        match app_state
            .quiz_question_service
            .create_quiz_question(new_quiz_question_dto)
            .await
        {
            Ok(quiz_quesion) => HttpResponse::Created().json(quiz_quesion),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[put("/{id}")]
    async fn update_quiz_question(
        app_state: Data<AppState>,
        path: Path<i32>,
        json: Json<UpdateQuizQuestionDto>,
    ) -> impl Responder {
        let update_quiz_question_dto = json.into_inner();
        let id = path.into_inner();

        match app_state
            .quiz_question_service
            .update_quiz_question(id, update_quiz_question_dto)
            .await
        {
            Ok(updated_quiz_quiestion_dto) => HttpResponse::Ok().json(updated_quiz_quiestion_dto),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }

    #[delete("/{id}")]
    async fn delete_quiz_question(app_state: Data<AppState>, path: Path<i32>) -> impl Responder {
        let id = path.into_inner();

        match app_state.quiz_question_service.delete_by_id(id).await {
            Ok(_) => {
                HttpResponse::Ok().body(format!("Quzi question with id {id} successfully deleted."))
            }
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

pub fn quiz_questions_routes() -> Scope {
    scope("quiz_questions")
        .service(get_all_quiz_quesitons)
        .service(find_quiz_question_by_id)
        .service(create_quiz_question)
        .service(update_quiz_question)
        .service(delete_quiz_question)
}
