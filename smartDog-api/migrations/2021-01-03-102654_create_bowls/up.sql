-- Your SQL goes here

CREATE TABLE bowls
(
    id   VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    waterlevel INTEGER NOT NULL,
    timestamp DATETIME NOT NULL
)