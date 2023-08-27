use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{Identifiable, QueryId, Queryable, Selectable, prelude::Insertable, query_builder::AsChangeset};


#[derive(Debug, Queryable, Selectable, QueryId, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: String,
    pub name: String,    
    pub insered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}


#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::brands)]
pub struct BrandInput {
    pub id: String,
    pub name: String,    
}