// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Integer,
        name -> Text,
        group_id -> Text,
        code -> Text,
        session_id -> Integer,
    }
}

diesel::table! {
    classes_student (id) {
        id -> Integer,
        class_id -> Integer,
        student_id -> Integer,
    }
}

diesel::table! {
    session (id) {
        id -> Integer,
        year -> Integer,
        session_name -> Text,
    }
}

diesel::table! {
    student (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        code -> Text,
    }
}

diesel::joinable!(classes -> session (session_id));
diesel::joinable!(classes_student -> classes (class_id));
diesel::joinable!(classes_student -> student (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    classes_student,
    session,
    student,
);
