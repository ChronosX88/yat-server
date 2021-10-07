table! {
    lists (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    lists_tasks (id) {
        id -> Nullable<Integer>,
        list_id -> Nullable<Integer>,
        task_id -> Integer,
    }
}

table! {
    tasks (id) {
        id -> Nullable<Integer>,
        name -> Text,
        user_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        due_date -> Nullable<Timestamp>,
        reminders -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}

joinable!(lists -> users (user_id));
joinable!(lists_tasks -> lists (list_id));
joinable!(lists_tasks -> tasks (task_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    lists,
    lists_tasks,
    tasks,
    users,
);
