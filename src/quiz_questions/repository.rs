use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::{
    common::{app_state::DbPool, repository::Repository},
    database::{
        connection_provider::ConnectionProvider,
        schema::{quiz_questions, quizzes},
    },
    quiz_questions::models::{NewQuizQuestion, QuizQuestion, UpdateQuizQuestion},
};

pub trait QuizQuestionRepository:
    Repository<QuizQuestion, NewQuizQuestion, UpdateQuizQuestion>
{
    fn get_by_quiz_id(&self, quiz_id: i32) -> Result<Vec<QuizQuestion>, String>;
    fn get_quiz_question_images_by_quiz_id(&self, quiz_id: i32) -> Result<Vec<String>, String>;
    fn get_quiz_question_images_by_user_id(&self, user_id: i32) -> Result<Vec<String>, String>;
}

#[derive(Debug)]
pub struct QuizQuestionRepositoryImpl {
    db: DbPool,
}

impl QuizQuestionRepositoryImpl {
    pub fn new(db: DbPool) -> Self {
        QuizQuestionRepositoryImpl { db }
    }
}

impl ConnectionProvider for QuizQuestionRepositoryImpl {
    fn pool(&self) -> &DbPool {
        &self.db
    }
}

impl QuizQuestionRepository for QuizQuestionRepositoryImpl {
    fn get_by_quiz_id(&self, quiz_id: i32) -> Result<Vec<QuizQuestion>, String> {
        let mut connection = self.get_connection()?;

        quiz_questions::table
            .filter(quiz_questions::quiz_id.eq(quiz_id))
            .load::<QuizQuestion>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn get_quiz_question_images_by_quiz_id(&self, quiz_id: i32) -> Result<Vec<String>, String> {
        let mut connection = self.get_connection()?;

        quiz_questions::table
            .filter(
                quiz_questions::quiz_id
                    .eq(quiz_id)
                    .and(quiz_questions::image_path.is_not_null()),
            )
            .select(quiz_questions::image_path)
            .load::<Option<String>>(&mut connection)
            .map(|images| images.into_iter().flatten().collect())
            .map_err(|error| error.to_string())
    }

    fn get_quiz_question_images_by_user_id(&self, user_id: i32) -> Result<Vec<String>, String> {
        let mut connection = self.get_connection()?;

        quiz_questions::table
            .inner_join(quizzes::table)
            .filter(
                quizzes::user_id
                    .eq(user_id)
                    .and(quiz_questions::image_path.is_not_null()),
            )
            .select(quiz_questions::image_path)
            .load::<Option<String>>(&mut connection)
            .map(|images| images.into_iter().flatten().collect())
            .map_err(|error| error.to_string())
    }
}

impl Repository<QuizQuestion, NewQuizQuestion, UpdateQuizQuestion> for QuizQuestionRepositoryImpl {
    fn get_all(&self) -> Result<Vec<QuizQuestion>, String> {
        let mut connection = self.get_connection()?;

        quiz_questions::table
            .load::<QuizQuestion>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn get_by_id(&self, id: i32) -> Result<Option<QuizQuestion>, String> {
        let mut connection = self.get_connection()?;

        quiz_questions::table
            .filter(quiz_questions::id.eq(id))
            .first::<QuizQuestion>(&mut connection)
            .optional()
            .map_err(|error| error.to_string())
    }

    fn create(&self, creating_enity: &NewQuizQuestion) -> Result<QuizQuestion, String> {
        let mut connection = self.get_connection()?;

        diesel::insert_into(quiz_questions::table)
            .values(creating_enity)
            .get_result::<QuizQuestion>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn update(
        &self,
        id: i32,
        updating_entity: &UpdateQuizQuestion,
    ) -> Result<QuizQuestion, String> {
        let mut connection = self.get_connection()?;

        diesel::update(quiz_questions::table.find(id))
            .set(updating_entity)
            .get_result::<QuizQuestion>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn delete(&self, id: i32) -> Result<usize, String> {
        let mut connection = self.get_connection()?;

        diesel::delete(quiz_questions::table.find(id))
            .execute(&mut connection)
            .map_err(|error| error.to_string())
    }
}
