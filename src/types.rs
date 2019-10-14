use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::*;
use std::fmt::Display;

#[derive(SqlType)]
#[postgres(type_name = "regconfig")]
pub struct Regconfig;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Regconfig"]
pub enum RegConfigEnum {
    English,
    Spanish,
}

impl Display for RegConfigEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            RegConfigEnum::English => f.write_str("English"),
            RegConfigEnum::Spanish => f.write_str("Spanish"),
        }
    }
}

impl FromSql<Regconfig, Pg> for RegConfigEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match u32::from_sql(bytes)? {
            13043 => Ok(RegConfigEnum::English),
            13063 => Ok(RegConfigEnum::Spanish),
            x => Err(format!("Unrecognized regconfig OID: {}", x).into()),
        }
    }
}

// TODO: implement serialization
