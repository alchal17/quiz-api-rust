# Quiz API

A REST API for creating and managing quizzes, built with [Actix-web](https://actix.rs/) and [Diesel](https://diesel.rs/) over PostgreSQL.

## Tech stack

- **Rust** (2024 edition)
- **Actix-web 4** — HTTP server / routing
- **Diesel 2** (`postgres`, `r2d2`) — ORM + connection pooling
- **PostgreSQL** — database
- **dotenv** — environment configuration
- **env_logger** — request/app logging

## Project structure

```
src/
├── main.rs                   # entry point, DB pool, server bootstrap
├── common/                    # shared app state, routing, blocking helpers
├── database/                  # connection provider, Diesel schema
├── users/                     # quiz_users: models, repository, service, routes
├── quizzes/                   # quizzes: models, repository, service, routes
├── quiz_questions/            # quiz_questions: models, repository, service, routes
├── quiz_question_option/      # quiz_question_options: models, repository, service, routes
└── images/                    # image upload/serving
uploads/                       # uploaded images (served at /images/{name})
```

Each domain module follows the same layering: `routes` (HTTP handlers) → `service` (business logic) → `repository` (Diesel queries) → `models` (DTOs / DB structs).

## Getting started

### Prerequisites

- Rust (stable, edition 2024 toolchain)
- PostgreSQL instance
- [Diesel CLI](https://diesel.rs/guides/getting-started) if you need to manage migrations:
  ```bash
  cargo install diesel_cli --no-default-features --features postgres
  ```

### Setup

1. Clone the repo:
   ```bash
   git clone https://github.com/alchal17/quiz-api-rust.git
   cd quiz-api-rust
   ```
2. Copy the env template and fill in your own values:
   ```bash
   cp .env.example .env
   ```
   | Variable       | Description                              |
   | -------------- | ---------------------------------------- |
   | `DATABASE_URL` | PostgreSQL connection string             |
   | `RUST_LOG`     | Log level (e.g. `info`, `debug`)         |

3. Make sure your database matches the schema in `src/database/schema.rs` (tables: `quiz_users`, `quizzes`, `quiz_questions`, `quiz_question_options`).

4. Run the server:
   ```bash
   cargo run
   ```
   The API listens on `0.0.0.0:8080` by default (see `src/common/constants.rs`). Uploaded images are stored in and served from `./uploads`.

## API overview

| Resource | Base path |
| --- | --- |
| Users | `/quiz_user` |
| Quizzes | `/quiz` |
| Quiz questions | `/quiz_questions` |
| Question options | `/quiz_question_option` |
| Images | `/images` |

### Users — `/quiz_user`

| Method | Path | Description |
| --- | --- | --- |
| GET | `/` | List all users |
| GET | `/{id}` | Get user by ID |
| GET | `/username/{username}` | Get user by username |
| GET | `/email/{email}` | Get user by email |
| POST | `/` | Create a new user |
| PUT | `/{id}` | Update a user |
| DELETE | `/{id}` | Delete a user |

### Quizzes — `/quiz`

| Method | Path | Description |
| --- | --- | --- |
| GET | `/` | List all quizzes |
| GET | `/{id}` | Get quiz by ID |
| POST | `/` | Create a new quiz |
| PUT | `/{id}` | Update a quiz |
| DELETE | `/{id}` | Delete a quiz |

### Quiz questions — `/quiz_questions`

| Method | Path | Description |
| --- | --- | --- |
| GET | `/` | List all quiz questions |
| GET | `/{id}` | Get question by ID |
| POST | `/` | Create a new question |
| PUT | `/{id}` | Update a question |
| DELETE | `/{id}` | Delete a question |

### Question options — `/quiz_question_option`

| Method | Path | Description |
| --- | --- | --- |
| GET | `/` | List all question options |
| GET | `/{id}` | Get option by ID |
| GET | `/by_quiz_id/{id}` | List options for a given question |
| POST | `/` | Create a new option |
| PUT | `/{id}` | Update an option |
| DELETE | `/{id}` | Delete an option |

### Images — `/images`

| Method | Path | Description |
| --- | --- | --- |
| GET | `/{image_name}` | Fetch an uploaded image |

## License

No license specified yet.
