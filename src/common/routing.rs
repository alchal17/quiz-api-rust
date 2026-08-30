use crate::{
    images::routes::image_routes, quiz_question_option::routes::get_quiz_question_option_routes,
    quiz_questions::routes::quiz_questions_routes, quizzes::routes::quiz_routes,
    users::routes::user_routes,
};
use actix_web::web;
pub fn get_routing(cfg: &mut web::ServiceConfig) {
    cfg.service(user_routes());
    cfg.service(quiz_routes());
    cfg.service(image_routes());
    cfg.service(quiz_questions_routes());
    cfg.service(get_quiz_question_option_routes());
}
