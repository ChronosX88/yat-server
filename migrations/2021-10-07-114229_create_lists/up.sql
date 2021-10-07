-- Your SQL goes here
create table if not exists lists (
    id integer primary key,
    user_id integer not null,
    name text not null,
    description text,
    foreign key (user_id) references users(id) on delete cascade
);