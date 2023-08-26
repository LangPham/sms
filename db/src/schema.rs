// @generated automatically by Diesel CLI.

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
