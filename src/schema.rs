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
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
        permissions -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    users,
);
