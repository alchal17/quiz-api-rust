use diesel::{
    ExpressionMethods, OptionalExtension, RunQueryDsl,
    query_dsl::methods::{FilterDsl, FindDsl},
};

use crate::{
    common::{app_state::DbPool, repository::Repository},
    database::{connection_provider::ConnectionProvider, schema::quiz_question_options},
    quiz_question_option::models::{
        NewQuizQuestionOption, QuizQuestionOption, UpdateQuizQuestionOption,
    },
};

pub trait QuizQuestionOptionRepository:
    Repository<QuizQuestionOption, NewQuizQuestionOption, UpdateQuizQuestionOption>
{
    fn find_by_quiz_question_id(
        &self,
        quiz_question_id: i32,
    ) -> Result<Vec<QuizQuestionOption>, String>;
}

#[derive(Debug)]
pub struct QuizQuestionOptionRepositoryImpl {
    db: DbPool,
}
impl QuizQuestionOptionRepositoryImpl {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}
impl ConnectionProvider for QuizQuestionOptionRepositoryImpl {
    fn pool(&self) -> &DbPool {
        &self.db
    }
}
impl Repository<QuizQuestionOption, NewQuizQuestionOption, UpdateQuizQuestionOption>
    for QuizQuestionOptionRepositoryImpl
{
    fn get_all(&self) -> Result<Vec<QuizQuestionOption>, String> {
        let mut connection = self.get_connection()?;

        quiz_question_options::table
            .load::<QuizQuestionOption>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn get_by_id(&self, id: i32) -> Result<Option<QuizQuestionOption>, String> {
        let mut conneciton = self.get_connection()?;

        quiz_question_options::table
            .filter(quiz_question_options::id.eq(id))
            .first::<QuizQuestionOption>(&mut conneciton)
            .optional()
            .map_err(|error| error.to_string())
    }

    fn create(&self, creating_enity: &NewQuizQuestionOption) -> Result<QuizQuestionOption, String> {
        let mut connection = self.get_connection()?;

        diesel::insert_into(quiz_question_options::table)
            .values(creating_enity)
            .get_result::<QuizQuestionOption>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn update(
        &self,
        id: i32,
        updating_entity: &UpdateQuizQuestionOption,
    ) -> Result<QuizQuestionOption, String> {
        let mut connection = self.get_connection()?;

        diesel::update(quiz_question_options::table.find(id))
            .set(updating_entity)
            .get_result::<QuizQuestionOption>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn delete(&self, id: i32) -> Result<usize, String> {
        let mut connection = self.get_connection()?;

        diesel::delete(quiz_question_options::table.find(id))
            .execute(&mut connection)
            .map_err(|error| error.to_string())
    }
}

impl QuizQuestionOptionRepository for QuizQuestionOptionRepositoryImpl {
    fn find_by_quiz_question_id(
        &self,
        quiz_question_id: i32,
    ) -> Result<Vec<QuizQuestionOption>, String> {
        let mut connection = self.get_connection()?;

        quiz_question_options::table
            .filter(quiz_question_options::quiz_question_id.eq(quiz_question_id))
            .load::<QuizQuestionOption>(&mut connection)
            .map_err(|error| error.to_string())
    }
}
