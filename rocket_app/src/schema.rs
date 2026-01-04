// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
