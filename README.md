Diesel regconfig type
===

## Creation

First create a lib crate:
```
cargo new --lib diesel_regconfig
```

Next add `diesel` and `dotenv` as dependencies.


Create a database with Postgres version 11.2.

```
cp .env.example .env
```

Change the DATABASE_URL if you wish.