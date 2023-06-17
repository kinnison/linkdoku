//! SQL Types for Linkdoku models

use common::objects;
use diesel::{
    backend::Backend, deserialize::FromSql, pg::Pg, query_builder::QueryId, serialize::ToSql,
    sql_types::Text, AsExpression, FromSqlRow, SqlType,
};

use crate::schema::sql_types::Visibility as VisibilityType;

#[derive(Debug, FromSqlRow, AsExpression, SqlType)]
#[diesel(sql_type = VisibilityType)]
pub enum Visibility {
    Restricted,
    Public,
    Published,
}

impl<DB: Backend> ToSql<VisibilityType, DB> for Visibility
where
    str: ToSql<Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match *self {
            Visibility::Restricted => ("restricted").to_sql(out),
            Visibility::Public => ("public").to_sql(out),
            Visibility::Published => ("published").to_sql(out),
        }
    }
}

impl FromSql<VisibilityType, Pg> for Visibility {
    fn from_sql(
        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"restricted" => Ok(Self::Restricted),
            b"public" => Ok(Self::Public),
            b"published" => Ok(Self::Published),
            _ => Err("Unrecognised visibility variant".into()),
        }
    }
}

impl From<Visibility> for objects::Visibility {
    fn from(val: Visibility) -> Self {
        match val {
            Visibility::Restricted => objects::Visibility::Restricted,
            Visibility::Public => objects::Visibility::Public,
            Visibility::Published => objects::Visibility::Published,
        }
    }
}

impl From<objects::Visibility> for Visibility {
    fn from(value: objects::Visibility) -> Self {
        match value {
            objects::Visibility::Restricted => Visibility::Restricted,
            objects::Visibility::Public => Visibility::Public,
            objects::Visibility::Published => Visibility::Published,
        }
    }
}

impl QueryId for crate::schema::sql_types::Visibility {
    type QueryId = Self;

    const HAS_STATIC_QUERY_ID: bool = true;
}
