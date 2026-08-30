use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::database::schema::quizzes;
use crate::users::models::QuizUser;

#[derive(Debug, Insertable)]
#[diesel(table_name = quizzes)]
pub struct NewQuiz {
    pub name: String,
    pub user_id: i32,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(QuizUser, foreign_key = user_id))]
#[diesel(table_name = quizzes)]
pub struct Quiz {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

impl Quiz {
    pub fn to_quiz_dto(self) -> QuizDto {
        QuizDto {
            id: self.id,
            name: self.name,
            user_id: self.user_id,
            description: self.description,
            image_path: self.image_path,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewQuizDto {
    pub name: String,
    pub user_id: i32,
    pub description: Option<String>,
    pub base64_image: Option<String>,
}

impl NewQuizDto {
    pub fn to_new_quiz(self, new_image_path: Option<String>) -> NewQuiz {
        NewQuiz {
            name: self.name,
            user_id: self.user_id,
            description: self.description,
            image_path: new_image_path,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct QuizDto {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub description: Option<String>,
    pub image_path: Option<String>,
}

impl QuizDto {
    pub fn to_quiz(self) -> Quiz {
        Quiz {
            id: self.id,
            name: self.name,
            user_id: self.user_id,
            description: self.description,
            image_path: self.image_path,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct UpdateQuizDto {
    pub name: Option<String>,
    pub user_id: Option<i32>,
    pub description: Option<String>,
    pub base64_image: Option<String>,
}

impl UpdateQuizDto {
    pub fn to_update_quiz(self, new_image_path: Option<String>) -> UpdateQuiz {
        UpdateQuiz {
            name: self.name,
            user_id: self.user_id,
            description: self.description,
            image_path: new_image_path,
        }
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = quizzes)]
pub struct UpdateQuiz {
    pub name: Option<String>,
    pub user_id: Option<i32>,
    pub description: Option<String>,
    pub image_path: Option<String>,
}
