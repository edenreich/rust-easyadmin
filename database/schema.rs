use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    user_sessions (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    user_sessions,
);

