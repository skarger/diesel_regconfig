use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::*;
use std::fmt::Display;

#[derive(SqlType)]
#[postgres(type_name = "regconfig")]
pub struct Regconfig;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Regconfig"]
pub struct RegConfig(u32);

const ENGLISH: u32 = 13043;
const SPANISH: u32 = 13063;

impl Display for RegConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            RegConfig(ENGLISH) => f.write_str("English"),
            RegConfig(SPANISH) => f.write_str("Spanish"),
            _ => f.write_str("Unsupported Language"),
        }
    }
}

impl FromSql<Regconfig, Pg> for RegConfig {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match u32::from_sql(bytes)? {
            oid => Ok(RegConfig(oid)),
        }
    }
}

// TODO: implement serialization
