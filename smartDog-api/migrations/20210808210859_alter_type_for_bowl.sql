-- Add migration script here
alter table bowls drop name;
alter table bowls add name varchar(128) not null;