use super::custom_type::{ClassTypeEnum, StatusEnum};
use chrono::NaiveDateTime;
use diesel::{
    prelude::Insertable, query_builder::AsChangeset, Identifiable, QueryId, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, QueryId, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::classes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Class {
    pub id: Uuid,
    pub status: StatusEnum,
    pub campus_id: Uuid,
    pub name: String,
    pub class_type: ClassTypeEnum,
    pub learn_year: String,
    pub insered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::classes)]
pub struct ClassInput {
    pub campus_id: Uuid,
    pub name: String,
    pub status: StatusEnum,
    pub class_type: ClassTypeEnum,
    pub learn_year: String,
}
