use chrono::NaiveDateTime;
use diesel::{
    prelude::Insertable, query_builder::AsChangeset, Identifiable, QueryId, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::custom_type::StatusEnum;

#[derive(Debug, Queryable, Selectable, QueryId, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::campuses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Campus {
    pub id: Uuid,
    pub brand_id: String,
    pub name: String,
    pub address: String,
    pub status: StatusEnum,
    pub insered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::campuses)]
pub struct CampusInput {
    pub brand_id: String,
    pub name: String,
    pub address: String,
    pub status: StatusEnum,
}
