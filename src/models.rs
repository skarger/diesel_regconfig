use super::types::RegConfigEnum;

#[derive(Queryable)]
pub struct RegConfig {
    pub id: i32,
    pub ts_config_name: RegConfigEnum,
}
