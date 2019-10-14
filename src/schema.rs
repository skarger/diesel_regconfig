table! {
    use diesel::sql_types::*;
    use crate::types::*;

    example_rows (id) {
        id -> Int4,
        ts_config_name -> Regconfig,
    }
}
