use std::sync::Arc;

use crate::{
    common::{blocking::run_blocking, constants::IMAGE_DIR},
    images::image_manager::{delete_image, save_base64_image},
    quiz_questions::{
        models::{NewQuizQuestionDto, QuizQuestionDto, UpdateQuizQuestionDto},
        repository::QuizQuestionRepository,
    },
};

pub struct QuizQuestionService<T: QuizQuestionRepository + 'static> {
    repo: Arc<T>,
}

impl<T: QuizQuestionRepository + 'static> QuizQuestionService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        QuizQuestionService { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<QuizQuestionDto>, String> {
        let repo = self.repo.clone();
        let quiz_questions = run_blocking(move || repo.get_all()).await?;

        Ok(quiz_questions
            .into_iter()
            .map(|quiz_question| quiz_question.to_quiz_question_dto())
            .collect())
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<QuizQuestionDto>, String> {
        let repo = self.repo.clone();
        let optional_quiz_question = run_blocking(move || repo.get_by_id(id)).await?;

        Ok(optional_quiz_question.map(|quiz_question| quiz_question.to_quiz_question_dto()))
    }

    pub async fn create_quiz_question(
        &self,
        new_quiz_question_dto: NewQuizQuestionDto,
    ) -> Result<QuizQuestionDto, String> {
        let repo = self.repo.clone();
        let created_quiz_question = run_blocking(move || {
            let image_path = new_quiz_question_dto
                .base64_image
                .as_ref()
                .map(|image| save_base64_image(image, IMAGE_DIR))
                .transpose()?;

            repo.create(&new_quiz_question_dto.to_new_quiz_question(image_path))
        })
        .await?;

        Ok(created_quiz_question.to_quiz_question_dto())
    }

    pub async fn update_quiz_question(
        &self,
        id: i32,
        update_quiz_questions_dto: UpdateQuizQuestionDto,
    ) -> Result<QuizQuestionDto, String> {
        let repo = self.repo.clone();
        let updated_quiz = run_blocking(move || {
            let existing_quiz_question = repo.get_by_id(id)?;
            if let Some(image_path) =
                existing_quiz_question.and_then(|quiz_question| quiz_question.image_path)
                && let Err(error) = delete_image(&image_path, IMAGE_DIR)
            {
                log::warn!("Failed to delete image {image_path}: {error}");
            }

            let created_image_path_optional = update_quiz_questions_dto
                .base64_image
                .as_ref()
                .map(|image| save_base64_image(image, IMAGE_DIR))
                .transpose()?;

            repo.update(
                id,
                &update_quiz_questions_dto.to_update_quiz_question(created_image_path_optional),
            )
        })
        .await?;

        Ok(updated_quiz.to_quiz_question_dto())
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<(), String> {
        let repo = self.repo.clone();
        run_blocking(move || {
            let Some(quiz_question_by_id) = repo.get_by_id(id)? else {
                return Err(format!("No quiz question with id {id} has been found"));
            };

            if let Some(image_path) = quiz_question_by_id.image_path
                && let Err(error) = delete_image(&image_path, IMAGE_DIR)
            {
                log::warn!("Failed to delete image {image_path}: {error}");
            }

            repo.delete(id)?;

            Ok(())
        })
        .await
    }
}
