use rocket_contrib::json::Json;
use rocket::response::status;
use crate::models::{Project, NewProject, Task, NewTask};
use crate::db_connection::DbConnection;
use rocket::Request;


#[get("/")]
pub fn list_projects(connection: DbConnection) -> Json<Vec<Project>> {
    let result = Project::index(connection);

    Json(result)
}

#[get("/<project_id>")]
pub fn show_project(connection: DbConnection, project_id: i32) -> Json<Project> {
    let result = Project::show(connection, project_id);

    Json(result)
}

#[post("/", data = "<new_project>")]
pub fn create_project(connection: DbConnection, new_project: Json<NewProject>) -> Json<Project> {
    let result = Project::create(connection, new_project);

    Json(result)
}

#[put("/<project_id>", data = "<updated_project>")]
pub fn update_project(connection: DbConnection, project_id: i32, updated_project: Json<NewProject>) -> Json<Project> {
    let result = Project::update(connection, project_id, updated_project);

    Json(result)
}

#[delete("/<project_id>")]
pub fn delete_project(connection: DbConnection, project_id: i32) -> status::NoContent {
    Project::destroy(connection, project_id);

    status::NoContent
}


#[get("/<project_id>/tasks")]
pub fn list_tasks(connection: DbConnection, project_id: i32) -> Json<Vec<Task>> {
    let result = Task::index(connection, project_id);

    Json(result)
}

#[get("/<project_id>/tasks/<task_id>")]
pub fn show_task(connection: DbConnection, project_id: i32, task_id: i32) -> Json<Task> {
    let result = Task::show(connection, project_id, task_id);

    Json(result)
}

#[post("/<project_id>/tasks", data = "<new_task>")]
pub fn create_task(connection: DbConnection, project_id: i32, new_task: Json<NewTask>) -> Json<Task> {
    let result = Task::create(connection, project_id, new_task);

    Json(result)
}

#[put("/<project_id>/tasks/<task_id>", data = "<updated_task>")]
pub fn update_task(connection: DbConnection, project_id: i32, task_id: i32, updated_task: Json<NewTask>) -> Json<Task> {
    let result = Task::update(connection, project_id, task_id, updated_task);

    Json(result)
}

#[delete("/<project_id>/tasks/<task_id>")]
pub fn delete_task(connection: DbConnection, project_id: i32, task_id: i32) -> status::NoContent {
    Task::destroy(connection, project_id, task_id);

    status::NoContent
}


#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
