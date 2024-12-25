// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Integer,
        name -> Text,
        group_id -> Text,
    }
}
