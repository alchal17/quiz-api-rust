use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, RunQueryDsl};

use crate::common::app_state::DbPool;
use crate::common::repository::Repository;
use crate::database::connection_provider::ConnectionProvider;
use crate::database::schema::quiz_users;
use crate::users::models::{NewQuizUser, QuizUser, UpdateQuizUser};

pub trait UserRepository: Repository<QuizUser, NewQuizUser, UpdateQuizUser> {
    fn find_by_username(&self, username: &str) -> Result<Option<QuizUser>, String>;
    fn find_by_email(&self, email: &str) -> Result<Option<QuizUser>, String>;
    fn find_another_by_username(&self, id: i32, username: &str)
    -> Result<Option<QuizUser>, String>;
    fn find_another_by_email(&self, id: i32, email: &str) -> Result<Option<QuizUser>, String>;
}
#[derive(Debug)]
pub struct UserRepositoryImpl {
    db: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(db: DbPool) -> Self {
        UserRepositoryImpl { db }
    }
}

impl ConnectionProvider for UserRepositoryImpl {
    fn pool(&self) -> &DbPool {
        &self.db
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find_by_email(&self, email: &str) -> Result<Option<QuizUser>, String> {
        let mut connection = self.get_connection()?;

        quiz_users::table
            .filter(quiz_users::email.eq(email))
            .first::<QuizUser>(&mut connection)
            .optional()
            .map_err(|error| format!("Error finding user by email {}: {}", email, error))
    }
    fn find_by_username(&self, username: &str) -> Result<Option<QuizUser>, String> {
        let mut connection = self.get_connection()?;

        quiz_users::table
            .filter(quiz_users::username.eq(username))
            .first::<QuizUser>(&mut connection)
            .optional()
            .map_err(|error| format!("Error finding user by username {}: {}", username, error))
    }
    fn find_another_by_email(&self, id: i32, email: &str) -> Result<Option<QuizUser>, String> {
        let mut connection = self.get_connection()?;

        quiz_users::table
            .filter((quiz_users::id.ne(id)).and(quiz_users::email.eq(email)))
            .first::<QuizUser>(&mut connection)
            .optional()
            .map_err(|error| {
                format!(
                    "Error finding another user by email {} and id {}: {}",
                    email, id, error
                )
            })
    }
    fn find_another_by_username(
        &self,
        id: i32,
        username: &str,
    ) -> Result<Option<QuizUser>, String> {
        let mut connection = self.get_connection()?;

        quiz_users::table
            .filter((quiz_users::id.ne(id)).and(quiz_users::username.eq(username)))
            .first::<QuizUser>(&mut connection)
            .optional()
            .map_err(|error| {
                format!(
                    "Error finding another user by username {} and id {}: {}",
                    username, id, error
                )
            })
    }
}

impl Repository<QuizUser, NewQuizUser, UpdateQuizUser> for UserRepositoryImpl {
    fn create(&self, creating_enity: &NewQuizUser) -> Result<QuizUser, String> {
        let mut connection = self.get_connection()?;

        diesel::insert_into(quiz_users::table)
            .values(creating_enity)
            .get_result::<QuizUser>(&mut connection)
            .map_err(|error| format!("Failed to create user: {}", error))
    }
    fn get_by_id(&self, id: i32) -> Result<Option<QuizUser>, String> {
        let mut connection = self.get_connection()?;

        quiz_users::table
            .filter(quiz_users::id.eq(id))
            .first::<QuizUser>(&mut connection)
            .optional()
            .map_err(|error| format!("Error finding a user with id {} :{}", id, error))
    }
    fn get_all(&self) -> Result<Vec<QuizUser>, String> {
        let mut connection = self.get_connection()?;

        quiz_users::table
            .load::<QuizUser>(&mut connection)
            .map_err(|error| format!("Error loading users: {}", error))
    }

    fn update(&self, id: i32, updating_entity: &UpdateQuizUser) -> Result<QuizUser, String> {
        let mut connection = self.get_connection()?;

        diesel::update(quiz_users::table.find(id))
            .set(updating_entity)
            .get_result::<QuizUser>(&mut connection)
            .map_err(|error| format!("Failed to update user with id {}: {}", id, error))
    }
    fn delete(&self, id: i32) -> Result<usize, String> {
        let mut connection = self.get_connection()?;

        diesel::delete(quiz_users::table.find(id))
            .execute(&mut connection)
            .map_err(|error| format!("Error deleting user with id {}: {}", id, error))
    }
}
