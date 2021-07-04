use crate::models::{Project, NewProject, Task, NewTask};
use crate::schema::{projects, tasks};
use crate::db_connection::DbConnection;
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


impl Task {
    pub fn index(db_connection: DbConnection, project_id: i32) -> Vec<Task> {
        let project: Project = projects::table.find(project_id).get_result::<Project>(&*db_connection).unwrap();

        Task::belonging_to(&project)
            .select((
                tasks::columns::id,
                tasks::columns::title,
                tasks::columns::project_id,
                tasks::columns::description,
            ))
            .load::<Task>(&*db_connection)
            .unwrap()
    }

    pub fn create(db_connection: DbConnection, project_id: i32, params: Json<NewTask>) -> Task {
        diesel::insert_into(tasks::table)
            .values((
                tasks::columns::title.eq(params.0.title),
                tasks::columns::description.eq(params.0.description),
                tasks::columns::project_id.eq(project_id),
            ))
            .get_result::<Task>(&*db_connection)
            .unwrap()
    }

    pub fn show(db_connection: DbConnection, project_id: i32, task_id: i32) -> Task {
        let project: Project = projects::table.find(project_id).get_result::<Project>(&*db_connection).unwrap();

        Task::belonging_to(&project)
            .select((
                tasks::columns::id,
                tasks::columns::title,
                tasks::columns::project_id,
                tasks::columns::description,
            ))
            .find(task_id)
            .get_result::<Task>(&*db_connection)
            .unwrap()
    }

    pub fn update(db_connection: DbConnection, project_id: i32, task_id: i32, params: Json<NewTask>) -> Task {
        let task = tasks::table
            .filter(tasks::columns::id.eq(task_id))
            .filter(tasks::columns::project_id.eq(project_id));

        diesel::update(task)
            .set((
                tasks::columns::title.eq(&params.title),
                tasks::columns::description.eq(&params.description),
                tasks::columns::project_id.eq(project_id)
            ))
            .get_result(&*db_connection)
            .unwrap()
    }

    pub fn destroy(db_connection: DbConnection, project_id: i32, task_id: i32) {
        let task = tasks::table
            .filter(tasks::columns::id.eq(task_id))
            .filter(tasks::columns::project_id.eq(project_id));

        diesel::delete(task).execute(&*db_connection);
    }
}