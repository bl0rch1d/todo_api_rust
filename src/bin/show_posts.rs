extern crate todo_api_rust;
extern crate diesel;

use self::todo_api_rust::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use todo_api_rust::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(false))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
        println!("\n\n")
    }
}
