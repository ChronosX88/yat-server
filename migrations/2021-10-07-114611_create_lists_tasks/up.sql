-- Your SQL goes here
create table if not exists lists_tasks (
    id integer primary key,
    list_id integer,
    task_id integer not null,
    foreign key (list_id) references lists(id) on delete set null,
    foreign key (task_id) references tasks(id) on delete cascade
);