// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "EClassType"))]
    pub struct EClassType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "EStatus"))]
    pub struct EStatus;
}

diesel::table! {
    brands (id) {
        id -> Varchar,
        name -> Varchar,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EStatus;

    campuses (id) {
        id -> Uuid,
        brand_id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        status -> EStatus,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EStatus;
    use super::sql_types::EClassType;

    classes (id) {
        id -> Uuid,
        status -> EStatus,
        campus_id -> Uuid,
        name -> Varchar,
        class_type -> EClassType,
        learn_year -> Varchar,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    peoples (id) {
        id -> Uuid,
        full_name -> Varchar,
        birthday -> Date,
        birthplace -> Varchar,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EStatus;

    student_class (id) {
        id -> Uuid,
        student_id -> Uuid,
        class_id -> Uuid,
        status -> EStatus,
        join_at -> Date,
        leave_at -> Nullable<Date>,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EStatus;

    students (id) {
        id -> Uuid,
        people_id -> Uuid,
        status -> EStatus,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(campuses -> brands (brand_id));
diesel::joinable!(classes -> campuses (campus_id));
diesel::joinable!(student_class -> classes (class_id));
diesel::joinable!(student_class -> students (student_id));
diesel::joinable!(students -> peoples (people_id));

diesel::allow_tables_to_appear_in_same_query!(
    brands,
    campuses,
    classes,
    peoples,
    student_class,
    students,
);
