use std::sync::Arc;

use crate::quiz_question_option::{
    models::{NewQuizQuestionOptionDto, QuizQuestionOptionDto, UpdateQuizQuestionOptionDto},
    repository::QuizQuestionOptionRepository,
};

pub struct QuizQuestionOptionService<T: QuizQuestionOptionRepository + 'static> {
    repo: Arc<T>,
}

impl<T: QuizQuestionOptionRepository + 'static> QuizQuestionOptionService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        QuizQuestionOptionService { repo }
    }

    pub fn get_quiz_question_option_by_id(
        &self,
        id: i32,
    ) -> Result<Option<QuizQuestionOptionDto>, String> {
        let optional_quiz_question_option = self.repo.get_by_id(id)?;

        if let Some(quiz_question_option) = optional_quiz_question_option {
            return Ok(Some(quiz_question_option.to_quiz_question_option_dto()));
        }

        Ok(None)
    }

    pub fn get_all_quiz_question_options(&self) -> Result<Vec<QuizQuestionOptionDto>, String> {
        let quiz_question_options = self.repo.get_all()?;

        Ok(quiz_question_options
            .into_iter()
            .map(|quiz_question_option| quiz_question_option.to_quiz_question_option_dto())
            .collect())
    }
    pub fn get_quiz_quesiton_options_by_quiz_question_id(
        &self,
        quiz_question_id: i32,
    ) -> Result<Vec<QuizQuestionOptionDto>, String> {
        let quiz_question_options = self.repo.find_by_quiz_question_id(quiz_question_id)?;

        Ok(quiz_question_options
            .into_iter()
            .map(|option| option.to_quiz_question_option_dto())
            .collect())
    }

    pub fn create_quiz_question_option(
        &self,
        new_quiz_question_option_dto: NewQuizQuestionOptionDto,
    ) -> Result<QuizQuestionOptionDto, String> {
        let new_quiz_question_option = new_quiz_question_option_dto.to_new_quiz_question_option();

        let created_option = self.repo.create(&new_quiz_question_option)?;
        Ok(created_option.to_quiz_question_option_dto())
    }

    pub fn update_quiz_question_option(
        &self,
        id: i32,
        update_quiz_question_option_dto: UpdateQuizQuestionOptionDto,
    ) -> Result<QuizQuestionOptionDto, String> {
        let update_option = update_quiz_question_option_dto.to_update_quiz_quesiton();

        let updated_option = self.repo.update(id, &update_option)?;

        Ok(updated_option.to_quiz_question_option_dto())
    }

    pub fn delete_quiz_question_option(&self, id: i32) -> Result<(), String> {
        match self.repo.get_by_id(id) {
            Ok(_) => {
                let _ = self.repo.delete(id)?;

                Ok(())
            }
            Err(err) => return Err(err),
        }
    }
}
