table! {
    deals (id) {
        id -> Int4,
        buyer_id -> Nullable<Int4>,
        seller_id -> Nullable<Int4>,
        house_id -> Nullable<Int4>,
        access_code -> Varchar,
        status -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        title -> Varchar,
    }
}

table! {
    houses (id) {
        id -> Int4,
        address -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        google_address -> Nullable<Jsonb>,
    }
}

table! {
    profiles (id) {
        id -> Int4,
        uid -> Int4,
        title -> Varchar,
        intro -> Text,
        body -> Text,
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
        roles -> Array<Text>,
    }
}

joinable!(deals -> houses (house_id));
joinable!(profiles -> users (uid));
joinable!(sessions -> users (uid));

allow_tables_to_appear_in_same_query!(
    deals,
    houses,
    profiles,
    sessions,
    users,
);
