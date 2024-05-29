// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Int4,
        filepath -> Varchar,
        size -> Int4,
        file_date -> Timestamp,
        replicas -> Varchar,
        to_delete -> Bool,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    replicas (id) {
        id -> Int4,
        name -> Varchar,
        ip -> Varchar,
        port -> Int4,
        connected_at -> Timestamp,
        is_online -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    replicas,
);
