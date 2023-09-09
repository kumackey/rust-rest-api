// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        users_id -> Int4,
        questions_id -> Int4,
        answer -> Varchar,
        answered_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        question -> Varchar,
        answer -> Varchar,
        questioner_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(answers -> questions (questions_id));
diesel::joinable!(answers -> users (users_id));
diesel::joinable!(questions -> users (questioner_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
    users,
);
