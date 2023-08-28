use super::custom_type::StatusEnum;
use chrono::NaiveDateTime;
use diesel::{
    prelude::Insertable, query_builder::AsChangeset, Identifiable, QueryId, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, QueryId, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
    pub id: Uuid,
    pub people_id: Uuid,
    pub status: StatusEnum,
    pub insered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::students)]
pub struct StudentInput {
    pub people_id: Uuid,
    pub status: StatusEnum,
}
