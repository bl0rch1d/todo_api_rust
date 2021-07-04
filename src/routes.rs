use rocket_contrib::json::Json;
use rocket::response::status;
use crate::models::{Project, NewProject};
use crate::db_connection::DbConnection;


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
