// @generated automatically by Diesel CLI.

diesel::table! {
    quiz_question_options (id) {
        id -> Int4,
        #[max_length = 100]
        text -> Varchar,
        is_correct -> Bool,
        quiz_question_id -> Int4,
    }
}

diesel::table! {
    quiz_questions (id) {
        id -> Int4,
        quiz_id -> Int4,
        #[max_length = 255]
        text -> Varchar,
        #[max_length = 100]
        image_path -> Nullable<Varchar>,
        multiple_choices -> Bool,
        seconds_answer -> Int4,
        order_number -> Int4,
    }
}

diesel::table! {
    quiz_users (id) {
        id -> Int4,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 50]
        email -> Varchar,
    }
}

diesel::table! {
    quizzes (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Int4,
        description -> Nullable<Text>,
        #[max_length = 100]
        image_path -> Nullable<Varchar>,
    }
}

diesel::joinable!(quiz_question_options -> quiz_questions (quiz_question_id));
diesel::joinable!(quiz_questions -> quizzes (quiz_id));
diesel::joinable!(quizzes -> quiz_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    quiz_question_options,
    quiz_questions,
    quiz_users,
    quizzes,
);
