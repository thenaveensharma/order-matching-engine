// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Uuid,
        token_id -> Int4,
        order_type -> Text,
        price -> Numeric,
        amount -> Numeric,
        status -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
