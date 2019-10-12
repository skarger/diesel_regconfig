pub mod types {
    #[derive(SqlType)]
    #[postgres(oid = "3734", array_oid = "3735")]
    pub enum Regconfig {
        English,
        Spanish,
    }
}