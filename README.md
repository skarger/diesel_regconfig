Diesel regconfig type
===

<!-- toc -->

- [Intro](#intro)
- [Implementation](#implementation)
- [Resources](#resources)
- [How to Install](#how-to-install)

<!-- tocstop -->

## Intro

This repo exists to demonstrate how to support a Postgres-specific column type, `regconfig`, in [Diesel](http://diesel.rs/).

The `regconfig` type relates to the Postgres [Full Text Search](https://www.postgresql.org/docs/current/textsearch.html) feature.
It represents the natural language of a searchable text document, like `english`.

When querying a database with a client like `psql`, the values of `regconfig` columns
basically appear as strings. Given a `ts_config_name` column of type `regconfig`,
a query would look like this:

```shell script
SELECT id, ts_config_name FROM example_rows;

ID  TS_CONFIG_NAME
1   english
2   spanish
```

However, unlike arbitrary strings, the only valid values are the identifiers
for languages supported in your database.
In fact, `regconfig` is one of several aliases
for a Postgres [Object Identifier](https://www.postgresql.org/docs/current/datatype-oid.html).
As it states on those Postgres docs,
> The oid type is currently implemented as an unsigned four-byte integer.

This is relevant to the Diesel implementation, as it means that we are *not*
reading text from the DB, but rather `u32` values.

This query obtains the available `regconfig` values your DB:

```sql
SELECT oid, * FROM pg_ts_config;

  oid  |  cfgname
-------+------------
  3748 | simple
 13039 | danish
 13041 | dutch
 13043 | english
...
```

The challenge is how to represent a `regconfig` column in Diesel:
* How to reference it in the Diesel `schema.rs` and `models.rs` modules.
* How to serialize and deserialize it.
* How to query and insert it from Rust code.

## Implementation

First, we have a Diesel migration to create an example table:

```sql
CREATE TABLE example_rows (
    id SERIAL PRIMARY KEY,
    ts_config_name REGCONFIG NOT NULL DEFAULT 'english'
);
```

After running this migration, `schema.rs` will look like this:

```rust
table! {
    regconfigs (id) {
        id -> Int4,
        ts_config_name -> Regconfig,
    }
}
```

However, the generated `Regconfig` reference is invalid - this code will not compile:

```shell script
error[E0412]: cannot find type `Regconfig` in this scope
 --> src/schema.rs:4:27
  |
4 |         ts_config_name -> Regconfig,
  |                           ^^^^^^^^^ not found in this scope
```

Commenting that line in `schema.rs` will allow the project to compile.
Of course then it's impossible to reference that column from Rust code.

When the Diesel code translates the DB values into Rust values,
it can use any Rust type, for example `String`, `Enum`, or `Struct`.
Since the valid `regconfig` values are a set of `OIDs`, a
[tuple struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)
that wraps a `u32` value works.

We define the Rust type like so:

```rust
#[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Regconfig"]
pub struct RegConfig(u32);
```

Notice the `#[sql_type = "Regconfig"]` macro. This refers to the custom SQL type
we define so that the Diesel-created `schema.rs` will have a valid reference:

```rust
    #[derive(SqlType)]
    #[postgres(type_name = "regconfig")]
    pub struct Regconfig;
```

Assuming that we define the above in `src/types.rs`, we must update `diesel.toml`
to tell `diesel_cli` to include it when dumping the schema:

```toml
import_types = ["diesel::sql_types::*", "crate::types::*"]
```

To make these types work, we must also implement the serialization/deserialization.

Implementing the [`FromSql`](https://docs.diesel.rs/diesel/deserialize/trait.FromSql.html) trait makes it possible to read this column from the DB:

```rust
impl FromSql<Regconfig, Pg> for RegConfig {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match u32::from_sql(bytes)? {
            oid => Ok(RegConfig(oid)),
        }
    }
}
```

Finally, we define a model that leverages the Rust type:

```rust
use super::types::RegConfig;

#[derive(Queryable)]
pub struct ExampleRow {
    pub id: i32,
    pub ts_config_name: RegConfig,
}
```

See `src/bin/` for a script that loads data from the DB into this model.

## Resources
* https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
* https://docs.diesel.rs/diesel/deserialize/trait.FromSql.html
* https://docs.diesel.rs/diesel/serialize/trait.ToSql.html

## How to Install

At the time of creating this repo, the current `diesel` version is `1.4.2`.

Locally I am using Postgres 11.2, although any version that supports [Full Text Search](https://www.postgresql.org/docs/current/textsearch.html) should work for this demo.

```
cp .env.example .env
```

Change the `DATABASE_URL` in the `.env` file if you wish, then:

```
diesel setup
```

```
diesel migration run
```

