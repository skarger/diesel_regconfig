Diesel regconfig type
===

## Creation

### How I created this repo

Created a lib crate:
```
cargo new --lib diesel_regconfig
```

Added `diesel` and `dotenv` as dependencies.

Created a database with Postgres version 11.2.

```
diesel migration generate create_regconfigs
```

### How to install locally
```
cp .env.example .env
```

Change the DATABASE_URL if you wish, then:

```
diesel setup
```

```
diesel migration run
```

