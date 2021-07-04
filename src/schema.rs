table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        project_id -> Int4,
        description -> Varchar,
    }
}

joinable!(tasks -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
);
