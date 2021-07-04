use crate::models::{Project, NewProject};
use crate::db_connection::DbConnection;
use crate::schema::projects;
use diesel::prelude::*;
use rocket_contrib::json::Json;

impl Project {
    pub fn index(db_connection: DbConnection) -> Vec<Project> {
        projects::table
            .order(projects::columns::id.desc())
            .load::<Project>(&*db_connection)
            .unwrap()
    }

    pub fn create(db_connection: DbConnection, params: Json<NewProject>) -> Project {
        diesel::insert_into(projects::table)
            .values(&*params)
            .get_result(&*db_connection)
            .unwrap()
    }

    pub fn show(db_connection: DbConnection, id: i32) -> Project {
        projects::table.find(id).first(&*db_connection).unwrap()
    }

    pub fn update(db_connection: DbConnection, id: i32, params: Json<NewProject>) -> Project {
        let project = projects::table.filter(projects::columns::id.eq(id));

        diesel::update(project)
            .set(projects::columns::title.eq(&params.title))
            .get_result(&*db_connection)
            .unwrap()
    }

    pub fn destroy(db_connection: DbConnection, id: i32) {
        let project = projects::table.filter(projects::columns::id.eq(id));

        diesel::delete(project).execute(&*db_connection);
    }
}