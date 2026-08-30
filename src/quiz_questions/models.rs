use crate::database::schema::quiz_questions;
use crate::quizzes::models::Quiz;
use diesel::{
    Selectable,
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable)]
#[diesel(table_name = quiz_questions)]
pub struct NewQuizQuestion {
    pub quiz_id: i32,
    pub text: String,
    pub image_path: Option<String>,
    pub multiple_choices: bool,
    pub seconds_answer: i32,
    pub order_number: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Quiz, foreign_key = quiz_id))]
#[diesel(table_name = quiz_questions)]
pub struct QuizQuestion {
    pub id: i32,
    pub quiz_id: i32,
    pub text: String,
    pub image_path: Option<String>,
    pub multiple_choices: bool,
    pub seconds_answer: i32,
    pub order_number: i32,
}

#[derive(Deserialize, Debug)]
pub struct NewQuizQuestionDto {
    pub quiz_id: i32,
    pub text: String,
    pub base64_image: Option<String>,
    pub multiple_choices: bool,
    pub seconds_answer: i32,
    pub order_number: i32,
}

#[derive(Debug, Serialize)]
pub struct QuizQuestionDto {
    pub id: i32,
    pub quiz_id: i32,
    pub text: String,
    pub image_path: Option<String>,
    pub multiple_choices: bool,
    pub seconds_answer: i32,
    pub order_number: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = quiz_questions)]
pub struct UpdateQuizQuestion {
    pub quiz_id: i32,
    pub text: String,
    pub image_path: Option<String>,
    pub multiple_choices: bool,
    pub seconds_answer: i32,
    pub order_number: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizQuestionDto {
    pub quiz_id: i32,
    pub text: String,
    pub base64_image: Option<String>,
    pub multiple_choices: bool,
    pub seconds_answer: i32,
    pub order_number: i32,
}
impl NewQuizQuestionDto {
    pub fn to_new_quiz_question(self, image_path: Option<String>) -> NewQuizQuestion {
        NewQuizQuestion {
            quiz_id: self.quiz_id,
            text: self.text,
            image_path,
            multiple_choices: self.multiple_choices,
            seconds_answer: self.seconds_answer,
            order_number: self.order_number,
        }
    }
}

impl QuizQuestion {
    pub fn to_quiz_question_dto(self) -> QuizQuestionDto {
        QuizQuestionDto {
            id: self.id,
            quiz_id: self.quiz_id,
            text: self.text,
            image_path: self.image_path,
            multiple_choices: self.multiple_choices,
            seconds_answer: self.seconds_answer,
            order_number: self.order_number,
        }
    }
}

impl UpdateQuizQuestionDto {
    pub fn to_update_quiz_question(self, image_path: Option<String>) -> UpdateQuizQuestion {
        UpdateQuizQuestion {
            quiz_id: self.quiz_id,
            text: self.text,
            image_path,
            multiple_choices: self.multiple_choices,
            seconds_answer: self.seconds_answer,
            order_number: self.order_number,
        }
    }
}
