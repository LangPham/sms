use super::custom_type::StatusEnum;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{
    prelude::Insertable, query_builder::AsChangeset, Identifiable, QueryId, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, QueryId, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::student_class)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Student, foreign_key = student_id))]
#[diesel(belongs_to(Class, foreign_key = class_id))]
pub struct StudentClass {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub status: StatusEnum,
    pub join_at: NaiveDate,
    pub leave_at: Option<NaiveDate>,
    pub insered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::student_class)]
pub struct StudentClassInput {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub status: StatusEnum,
    pub join_at: NaiveDate,
    pub leave_at: Option<NaiveDate>,
}
