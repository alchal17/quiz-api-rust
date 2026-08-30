use diesel::{
    Selectable,
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    query_builder::AsChangeset,
};
use serde::{Deserialize, Serialize};

use crate::{database::schema::quiz_question_options, quiz_questions::models::QuizQuestion};

#[derive(Debug, Insertable)]
#[diesel(table_name = quiz_question_options)]
pub struct NewQuizQuestionOption {
    pub quiz_question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(QuizQuestion, foreign_key = quiz_question_id))]
#[diesel(table_name = quiz_question_options)]
pub struct QuizQuestionOption {
    pub id: i32,
    pub text: String,
    pub is_correct: bool,
    pub quiz_question_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewQuizQuestionOptionDto {
    pub quiz_question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize)]
pub struct QuizQuestionOptionDto {
    pub id: i32,
    pub quiz_question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = quiz_question_options)]
pub struct UpdateQuizQuestionOption {
    pub quiz_question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizQuestionOptionDto {
    pub quiz_question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

impl NewQuizQuestionOptionDto {
    pub fn to_new_quiz_question_option(self) -> NewQuizQuestionOption {
        NewQuizQuestionOption {
            quiz_question_id: self.quiz_question_id,
            text: self.text,
            is_correct: self.is_correct,
        }
    }
}

impl QuizQuestionOption {
    pub fn to_quiz_question_option_dto(self) -> QuizQuestionOptionDto {
        QuizQuestionOptionDto {
            id: self.id,
            quiz_question_id: self.quiz_question_id,
            text: self.text,
            is_correct: self.is_correct,
        }
    }
}

impl UpdateQuizQuestionOptionDto {
    pub fn to_update_quiz_quesiton(self) -> UpdateQuizQuestionOption {
        UpdateQuizQuestionOption {
            quiz_question_id: self.quiz_question_id,
            text: self.text,
            is_correct: self.is_correct,
        }
    }
}
