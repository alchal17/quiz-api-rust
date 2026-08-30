use futures::future::join_all;

use crate::{
    common::{blocking::run_blocking, constants::IMAGE_DIR},
    images::image_manager::delete_image,
    quiz_questions::repository::QuizQuestionRepository,
    quizzes::repository::QuizRepository,
    users::models::{NewQuizUserDto, QuizUserDto, UpdateQuizUserDto},
};

use super::repository::UserRepository;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserService<
    R: UserRepository + 'static,
    Q: QuizRepository + 'static,
    T: QuizQuestionRepository + 'static,
> {
    user_repository: Arc<R>,
    quiz_repository: Arc<Q>,
    quiz_question_repository: Arc<T>,
}

impl<R: UserRepository + 'static, Q: QuizRepository + 'static, T: QuizQuestionRepository + 'static>
    UserService<R, Q, T>
{
    pub fn new(
        user_repository: Arc<R>,
        quiz_repository: Arc<Q>,
        quiz_question_repository: Arc<T>,
    ) -> Self {
        UserService {
            user_repository,
            quiz_repository,
            quiz_question_repository,
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<QuizUserDto>, String> {
        let repo = self.user_repository.clone();
        let users = run_blocking(move || repo.get_all()).await?;

        Ok(users
            .into_iter()
            .map(|user| user.to_quiz_user_dto())
            .collect())
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<QuizUserDto>, String> {
        let repo = self.user_repository.clone();
        let option_user = run_blocking(move || repo.get_by_id(id)).await?;

        Ok(option_user.map(|user| user.to_quiz_user_dto()))
    }

    pub async fn create_new_user(&self, new_user: NewQuizUserDto) -> Result<QuizUserDto, String> {
        let repo = self.user_repository.clone();
        let new_user = run_blocking(move || repo.create(&new_user.to_new_quiz_user())).await?;

        Ok(new_user.to_quiz_user_dto())
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<QuizUserDto>, String> {
        let repo = self.user_repository.clone();
        let username = username.to_owned();
        let option_user = run_blocking(move || repo.find_by_username(&username)).await?;

        Ok(option_user.map(|user| user.to_quiz_user_dto()))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<QuizUserDto>, String> {
        let repo = self.user_repository.clone();
        let email = email.to_owned();
        let option_user = run_blocking(move || repo.find_by_email(&email)).await?;

        Ok(option_user.map(|user| user.to_quiz_user_dto()))
    }

    pub async fn update(
        &self,
        id: i32,
        update_quiz_user_dto: UpdateQuizUserDto,
    ) -> Result<QuizUserDto, String> {
        let repo = self.user_repository.clone();
        let user =
            run_blocking(move || repo.update(id, &update_quiz_user_dto.to_update_quiz_user()))
                .await?;

        Ok(user.to_quiz_user_dto())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), String> {
        let repo = self.user_repository.clone();
        let Some(user) = run_blocking(move || repo.get_by_id(id)).await? else {
            return Err(format!("No user with id {id} found"));
        };

        let quiz_repo = self.quiz_repository.clone();
        let quiz_question_repo = self.quiz_question_repository.clone();
        let user_id = user.id;

        let mut images_to_be_removed =
            run_blocking(move || quiz_repo.find_quiz_images_by_user_id(user_id)).await?;
        images_to_be_removed.extend(
            run_blocking(move || quiz_question_repo.get_quiz_question_images_by_user_id(user_id))
                .await?,
        );

        let repo = self.user_repository.clone();
        run_blocking(move || repo.delete(id)).await?;

        let images_to_be_removed_tasks = images_to_be_removed.into_iter().map(|image| {
            tokio::task::spawn_blocking(move || {
                if let Err(error) = delete_image(&image, IMAGE_DIR) {
                    log::warn!("Failed to delete image {image}: {error}");
                }
            })
        });
        join_all(images_to_be_removed_tasks).await;

        Ok(())
    }
}
