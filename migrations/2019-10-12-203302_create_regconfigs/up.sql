CREATE TABLE regconfigs (
    id SERIAL PRIMARY KEY,
    ts_config_name REGCONFIG NOT NULL DEFAULT 'english'
);