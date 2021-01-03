# SmartDog Simple API

## Prerequsits

diesel cli

```shell
cargo install diesel_cli --no-default-features --features sqllite
```

## Setup

```shell
echo DATABASE_URL=smartbowl.db > .env
diesel setup
diesel migration run
diesel migration redo -- if you need to redo/restart
```
In you `smartbowl.db` you should now have a table 
with the following layout:

```sqlite
CREATE TABLE bowls
(
    id   VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    waterlevel INTEGER NOT NULL,
    timestamp DATETIME NOT NULL
)
```