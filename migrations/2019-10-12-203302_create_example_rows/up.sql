CREATE TABLE example_rows (
    id SERIAL PRIMARY KEY,
    ts_config_name REGCONFIG NOT NULL DEFAULT 'english'
);