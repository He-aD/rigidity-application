table! {
    users (email) {
        email -> Varchar,
        nickname -> Varchar,
        hash -> Varchar,
        reset_password_hash -> Nullable<Varchar>,
        password_hash_expire_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}
