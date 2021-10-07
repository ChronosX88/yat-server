create table if not exists tasks (
    id integer primary key,
    name text not null,
    user_id integer not null,
    created_at timestamp not null,
    updated_at timestamp,
    description text,
    due_date timestamp,
    reminders text,
    foreign key (user_id) references users(id) on delete cascade
);