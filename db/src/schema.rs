// @generated automatically by Diesel CLI.

pub mod sql_types {
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
    peoples (id) {
        id -> Uuid,
        full_name -> Varchar,
        birthday -> Date,
        birthplace -> Varchar,
        insered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(campuses -> brands (brand_id));

diesel::allow_tables_to_appear_in_same_query!(brands, campuses, peoples,);
