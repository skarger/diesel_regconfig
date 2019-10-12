Diesel regconfig type
===

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

