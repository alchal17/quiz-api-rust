use std::sync::Arc;

use futures::future::join_all;

use crate::{
    common::{blocking::run_blocking, constants::IMAGE_DIR},
    images::image_manager::{delete_image, save_base64_image},
    quiz_questions::repository::QuizQuestionRepository,
    quizzes::{
        models::{NewQuizDto, QuizDto, UpdateQuizDto},
        repository::QuizRepository,
    },
};

pub struct QuizService<R: QuizRepository + 'static, Q: QuizQuestionRepository + 'static> {
    quiz_repo: Arc<R>,
    quiz_question_repo: Arc<Q>,
}

impl<R: QuizRepository + 'static, Q: QuizQuestionRepository + 'static> QuizService<R, Q> {
    pub fn new(quiz_repo: Arc<R>, quiz_question_repo: Arc<Q>) -> Self {
        QuizService {
            quiz_repo,
            quiz_question_repo,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<QuizDto>, String> {
        let repo = self.quiz_repo.clone();
        let quizzes = run_blocking(move || repo.get_all()).await?;

        Ok(quizzes.into_iter().map(|quiz| quiz.to_quiz_dto()).collect())
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<QuizDto>, String> {
        let repo = self.quiz_repo.clone();
        let optional_quiz = run_blocking(move || repo.get_by_id(id)).await?;

        Ok(optional_quiz.map(|quiz| quiz.to_quiz_dto()))
    }

    pub async fn create_quiz(&self, new_quiz_dto: NewQuizDto) -> Result<QuizDto, String> {
        let repo = self.quiz_repo.clone();
        let created_quiz = run_blocking(move || {
            let image_path = new_quiz_dto
                .base64_image
                .as_ref()
                .map(|image| save_base64_image(image, IMAGE_DIR))
                .transpose()?;

            repo.create(&new_quiz_dto.to_new_quiz(image_path))
        })
        .await?;

        Ok(created_quiz.to_quiz_dto())
    }

    pub async fn update_quiz(
        &self,
        id: i32,
        update_quiz: UpdateQuizDto,
    ) -> Result<QuizDto, String> {
        let repo = self.quiz_repo.clone();
        let updated_quiz = run_blocking(move || {
            let existing_quiz = repo.get_by_id(id)?;
            if let Some(image_path) = existing_quiz.and_then(|quiz| quiz.image_path)
                && let Err(error) = delete_image(&image_path, IMAGE_DIR)
            {
                log::warn!("Failed to delete image {image_path}: {error}");
            }

            let created_image_path_optional = update_quiz
                .base64_image
                .as_ref()
                .map(|image| save_base64_image(image, IMAGE_DIR))
                .transpose()?;

            repo.update(id, &update_quiz.to_update_quiz(created_image_path_optional))
        })
        .await?;

        Ok(updated_quiz.to_quiz_dto())
    }

    pub async fn delete_quiz(&self, id: i32) -> Result<(), String> {
        let repo = self.quiz_repo.clone();
        let Some(quiz) = run_blocking(move || repo.get_by_id(id)).await? else {
            return Err(format!("Quiz with id {} not found", id));
        };
        let quiz_id = quiz.id;
        let quiz_image_optional = quiz.image_path;

        let quiz_question_repo = self.quiz_question_repo.clone();
        let mut images_to_be_deleted =
            run_blocking(move || quiz_question_repo.get_quiz_question_images_by_quiz_id(quiz_id))
                .await?;

        if let Some(quiz_image) = quiz_image_optional {
            images_to_be_deleted.push(quiz_image);
        }

        let repo = self.quiz_repo.clone();
        run_blocking(move || repo.delete(id)).await?;

        let images_deletions_tasks = images_to_be_deleted.into_iter().map(|image| {
            tokio::task::spawn_blocking(move || {
                if let Err(error) = delete_image(&image, IMAGE_DIR) {
                    log::warn!("Failed to delete image {image}: {error}");
                }
            })
        });

        join_all(images_deletions_tasks).await;

        Ok(())
    }
}
