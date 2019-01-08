table! {
    sessions (id) {
        id -> Int4,
        uid -> Int4,
        created -> Timestamp,
        updated -> Timestamp,
        hash -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(sessions -> users (uid));

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
