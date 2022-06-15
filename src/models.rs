use diesel::{Queryable, Insertable, Associations, Identifiable};
use serde::{Serialize, Deserialize};
use crate::schema::{projects, tasks};


#[derive(Identifiable, Queryable, Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="projects"]
pub struct NewProject {
    pub title: String,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(Project)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub project_id: i32,
    pub description: String,
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="tasks"]
pub struct NewTask {
    pub title: String,
    pub description: String,
}
