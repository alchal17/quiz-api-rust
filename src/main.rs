use std::env;

use actix_web::{App, HttpServer, middleware::Logger, web};
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
mod common;
mod database;
use common::app_state::AppState;
use common::routing::get_routing;

use crate::common::app_state::DbPool;
use crate::common::constants::{HOST, IMAGE_DIR, PORT};

mod images;
mod quiz_question_option;
mod quiz_questions;
mod quizzes;
mod users;

fn get_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::fs::create_dir_all(IMAGE_DIR)?;

    let app_state = web::Data::new(AppState::new(get_db_pool()));
    println!("Server runs at https://{HOST}:{PORT}");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(get_routing)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
