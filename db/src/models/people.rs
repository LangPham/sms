use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::{Identifiable, QueryId, Queryable, Selectable, prelude::Insertable, query_builder::AsChangeset};
use uuid::Uuid;



#[derive(Debug, Queryable, Selectable, QueryId, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::peoples)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct People {
    pub id: Uuid,
    pub full_name: String,
    pub birthday: NaiveDate,
    pub birthplace: String,
    pub insered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}


#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::peoples)]
pub struct PeopleInput {
    pub full_name: String,
    pub birthday: NaiveDate,
    pub birthplace: String,
}