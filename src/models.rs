use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::projects;


#[derive(Queryable, Serialize)]
pub struct Project {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="projects"]
pub struct NewProject {
    pub title: String,
}
