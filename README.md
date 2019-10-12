Diesel regconfig type
===

This repo exists to figure out how to support a Postgres-specific column type, `regconfig` in [Diesel](http://diesel.rs/).

The `regconfig` type relates to the Postgres [Full Text Search](https://www.postgresql.org/docs/current/textsearch.html) feature.

It represents the natural language of a searchable text document, like `english`.

When querying a database with a client like `psql`, the values of `regconfig` columns
basically appear as strings. Given a `ts_config_name` column of type `regconfig`,
a query would look like this:

```
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

