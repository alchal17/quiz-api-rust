use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::{
    common::{app_state::DbPool, repository::Repository},
    database::{connection_provider::ConnectionProvider, schema::quizzes},
    quizzes::models::{NewQuiz, Quiz, UpdateQuiz},
};

pub trait QuizRepository: Repository<Quiz, NewQuiz, UpdateQuiz> {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Quiz>, String>;
    fn find_quiz_images_by_user_id(&self, user_id: i32) -> Result<Vec<String>, String>;
}

#[derive(Debug)]
pub struct QuizRepositoryImpl {
    db: DbPool,
}

impl QuizRepositoryImpl {
    pub fn new(db: DbPool) -> Self {
        QuizRepositoryImpl { db }
    }
}
impl ConnectionProvider for QuizRepositoryImpl {
    fn pool(&self) -> &DbPool {
        &self.db
    }
}
impl QuizRepository for QuizRepositoryImpl {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Quiz>, String> {
        let mut connection = self.get_connection()?;

        quizzes::table
            .filter(quizzes::user_id.eq(user_id))
            .load::<Quiz>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn find_quiz_images_by_user_id(&self, user_id: i32) -> Result<Vec<String>, String> {
        let mut connection = self.get_connection()?;

        quizzes::table
            .filter(quizzes::user_id.eq(user_id))
            .select(quizzes::image_path)
            .load::<Option<String>>(&mut connection)
            .map(|paths| paths.into_iter().flatten().collect())
            .map_err(|error| error.to_string())
    }
}

impl Repository<Quiz, NewQuiz, UpdateQuiz> for QuizRepositoryImpl {
    fn get_all(&self) -> Result<Vec<Quiz>, String> {
        let mut connection = self.get_connection()?;
        quizzes::table
            .load::<Quiz>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn get_by_id(&self, id: i32) -> Result<Option<Quiz>, String> {
        let mut connection = self.get_connection()?;

        quizzes::table
            .filter(quizzes::id.eq(id))
            .first::<Quiz>(&mut connection)
            .optional()
            .map_err(|error| error.to_string())
    }

    fn create(&self, creating_enity: &NewQuiz) -> Result<Quiz, String> {
        let mut connection = self.get_connection()?;

        diesel::insert_into(quizzes::table)
            .values(creating_enity)
            .get_result::<Quiz>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn update(&self, id: i32, updating_entity: &UpdateQuiz) -> Result<Quiz, String> {
        let mut connection = self.get_connection()?;

        diesel::update(quizzes::table.find(id))
            .set(updating_entity)
            .get_result::<Quiz>(&mut connection)
            .map_err(|error| error.to_string())
    }

    fn delete(&self, id: i32) -> Result<usize, String> {
        let mut connection = self.get_connection()?;

        diesel::delete(quizzes::table.find(id))
            .execute(&mut connection)
            .map_err(|error| error.to_string())
    }
}
