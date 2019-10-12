Diesel regconfig type
===

<!-- toc -->

- [Intro](#intro)
- [Initial attempts](#initial-attempts)
- [How to Install](#how-to-install)

<!-- tocstop -->

## Intro

This repo exists to figure out how to support a Postgres-specific column type, `regconfig` in [Diesel](http://diesel.rs/).

The `regconfig` type relates to the Postgres [Full Text Search](https://www.postgresql.org/docs/current/textsearch.html) feature.

It represents the natural language of a searchable text document, like `english`.

When querying a database with a client like `psql`, the values of `regconfig` columns
basically appear as strings. Given a `ts_config_name` column of type `regconfig`,
a query would look like this:

```shell script
SELECT id, ts_config_name FROM regconfigs;

ID  TS_CONFIG_NAME
1   english
2   spanish
```

However, unlike arbitrary strings, the only valid values are the identifiers for languages supported in your database.
Therefore a Rust `Enum` could be a good way to represent `regconfig` values, although a `String` would be acceptable as well.

The challenge is how to represent a `regconfig` column in Diesel:
* How to represent it in the Diesel `schema.rs` and `models.rs` modules.
* How to serialize and deserialize it.
* How to query and insert it from Rust code.

## Initial attempts

First, a Diesel migration to create an example table:

```sql
CREATE TABLE regconfigs (
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

Taking the `Enum` approach, I can define a type using Diesel's `#[derive(SqlType)]`:

```rust
pub mod types {
    #[derive(SqlType)]
    #[postgres(oid = "3734", array_oid = "3735")]
    pub enum Regconfig {
        English,
        Spanish,
    }
}
```

This compiles. The question is how to use it in `schema.rs`,
or if this is not a good approach, then what to do instead.

## How to Install

At the time of creating this repo, the current `diesel` version is `1.4.2`.

Locally I am using Postgres 11.2, although any version that supports [Full Text Search](https://www.postgresql.org/docs/current/textsearch.html) should work for this demo.

```
cp .env.example .env
```

Change the `DATABASE_URL` in that file if you wish, then:

```
diesel setup
```

```
diesel migration run
```

