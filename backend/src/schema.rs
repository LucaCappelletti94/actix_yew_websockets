// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        body -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
    }
}

diesel::joinable!(comments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    users,
);
