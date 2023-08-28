use crate::schema::sql_types::{EClassType, EStatus};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{IsNull, Output, ToSql},
    sql_types::SqlType,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(
    SqlType, Debug, Default, Clone, Serialize, Deserialize, PartialEq, FromSqlRow, AsExpression,
)]
#[diesel(sql_type = EStatus)]
pub enum StatusEnum {
    #[default]
    Draft,
    Active,
    Deleted,
    Archive,
}

impl ToSql<EStatus, Pg> for StatusEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            StatusEnum::Active => out.write_all(b"ACTIVE")?,
            StatusEnum::Draft => out.write_all(b"DRAFT")?,
            StatusEnum::Deleted => out.write_all(b"DELETED")?,
            StatusEnum::Archive => out.write_all(b"ARCHIVE")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<EStatus, Pg> for StatusEnum {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ACTIVE" => Ok(StatusEnum::Active),
            b"DRAFT" => Ok(StatusEnum::Draft),
            b"DELETED" => Ok(StatusEnum::Deleted),
            b"ARCHIVE" => Ok(StatusEnum::Archive),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(
    SqlType, Debug, Default, Clone, Serialize, Deserialize, PartialEq, FromSqlRow, AsExpression,
)]
#[diesel(sql_type = EClassType)]
pub enum ClassTypeEnum {
    #[default]
    Main,
    Extra,
}

impl ToSql<EClassType, Pg> for ClassTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            ClassTypeEnum::Main => out.write_all(b"MAIN")?,
            ClassTypeEnum::Extra => out.write_all(b"EXTRA")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<EClassType, Pg> for ClassTypeEnum {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"MAIN" => Ok(ClassTypeEnum::Main),
            b"EXTRA" => Ok(ClassTypeEnum::Extra),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
