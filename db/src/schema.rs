table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passhash -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}
