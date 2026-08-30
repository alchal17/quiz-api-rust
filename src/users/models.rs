use crate::database::schema::quiz_users;
use diesel::{
    AsChangeset, Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = quiz_users)]
pub struct QuizUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl QuizUser {
    pub fn to_quiz_user_dto(self) -> QuizUserDto {
        QuizUserDto {
            id: self.id,
            username: self.username,
            email: self.email,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = quiz_users)]
pub struct NewQuizUser {
    pub username: String,
    pub email: String,
}

#[derive(AsChangeset)]
#[diesel(table_name=quiz_users)]
pub struct UpdateQuizUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
}

impl UpdateQuizUserDto {
    pub fn to_update_quiz_user(self) -> UpdateQuizUser {
        UpdateQuizUser {
            username: self.username,
            email: self.email,
        }
    }
}

impl NewQuizUser {
    pub fn to_new_quiz_user_dto(self) -> NewQuizUserDto {
        NewQuizUserDto {
            username: self.username,
            email: self.email,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct QuizUserDto {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct NewQuizUserDto {
    pub username: String,
    pub email: String,
}

impl NewQuizUserDto {
    pub fn to_new_quiz_user(self) -> NewQuizUser {
        NewQuizUser {
            username: self.username,
            email: self.email,
        }
    }
}
