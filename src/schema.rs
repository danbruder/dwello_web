table! {
    deals (id) {
        id -> Int4,
        bid -> Nullable<Int4>,
        sid -> Nullable<Int4>,
        hid -> Nullable<Int4>,
        access_code -> Varchar,
        status -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    houses (id) {
        id -> Int4,
        address -> Varchar,
        lat -> Varchar,
        lon -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        uid -> Int4,
        token -> Text,
        active -> Bool,
        created -> Timestamp,
        updated -> Timestamp,
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

joinable!(deals -> houses (hid));
joinable!(sessions -> users (uid));

allow_tables_to_appear_in_same_query!(
    deals,
    houses,
    sessions,
    users,
);
