use super::types::RegConfig;

#[derive(Queryable)]
pub struct ExampleRow {
    pub id: i32,
    pub ts_config_name: RegConfig,
}
