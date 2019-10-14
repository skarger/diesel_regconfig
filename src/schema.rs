table! {
    use diesel::sql_types::*;
    use crate::types::*;

    regconfigs (id) {
        id -> Int4,
        ts_config_name -> Regconfig,
    }
}
