use std::sync::Arc;

use crate::{
    quiz_question_option::{
        repository::QuizQuestionOptionRepositoryImpl, service::QuizQuestionOptionService,
    },
    quiz_questions::{repository::QuizQuestionRepositoryImpl, service::QuizQuestionService},
    quizzes::{repository::QuizRepositoryImpl, service::QuizService},
    users::{repository::UserRepositoryImpl, service::UserService},
};
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub user_service:
        UserService<UserRepositoryImpl, QuizRepositoryImpl, QuizQuestionRepositoryImpl>,
    pub quiz_serice: QuizService<QuizRepositoryImpl, QuizQuestionRepositoryImpl>,
    pub quiz_question_service: QuizQuestionService<QuizQuestionRepositoryImpl>,
    pub quiz_question_option_service: QuizQuestionOptionService<QuizQuestionOptionRepositoryImpl>,
}

impl AppState {
    pub fn new(db_pool: DbPool) -> Self {
        let quiz_question_option_repository =
            Arc::new(QuizQuestionOptionRepositoryImpl::new(db_pool.clone()));
        let quiz_question_repository = Arc::new(QuizQuestionRepositoryImpl::new(db_pool.clone()));

        let quiz_question_option_service =
            QuizQuestionOptionService::new(quiz_question_option_repository);
        let quiz_question_service = QuizQuestionService::new(quiz_question_repository.clone());

        let quiz_repository = Arc::new(QuizRepositoryImpl::new(db_pool.clone()));
        let quiz_serice =
            QuizService::new(quiz_repository.clone(), quiz_question_repository.clone());

        let user_repository = Arc::new(UserRepositoryImpl::new(db_pool.clone()));
        let user_service = UserService::new(
            user_repository,
            quiz_repository.clone(),
            quiz_question_repository.clone(),
        );

        AppState {
            user_service,
            quiz_serice,
            quiz_question_service,
            quiz_question_option_service,
        }
    }
}
