use rocket_contrib::databases::{diesel::PgConnection};


#[database("postgres")]
pub struct DbConnection(PgConnection);
