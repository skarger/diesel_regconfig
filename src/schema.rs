use crate::types::types::Regconfig;

table! {
    use diesel::sql_types::*;
    use super::Regconfig;
    regconfigs {
        id -> Int4,
        ts_config_name -> Regconfig,
    }
}
