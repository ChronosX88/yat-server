-- Your SQL goes here
create table if not exists users (
    id integer primary key,
    created_at timestamp not null,
    updated_at timestamp,
    username text unique not null,
    email text unique not null,
    password text not null
);