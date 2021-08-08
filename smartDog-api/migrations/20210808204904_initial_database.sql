-- Add migration script here
create table bowls(
    id SERIAL PRIMARY KEY,
    name TEXT
);

create table history(
    id SERIAL PRIMARY KEY,
    waterbowl_id INTEGER NOT NULL,
    waterlevel INTEGER NOT NULL,
    timestamp TIMESTAMP
);
