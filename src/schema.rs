table! {
    files (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        path -> Text,
        uploader -> Text,
        date -> Text,
    }
}

table! {
    sessions (session_id) {
        session_id -> Text,
        expire_at -> Text,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
        permissions -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    sessions,
    users,
);
