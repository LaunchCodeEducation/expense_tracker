extern crate diesel_budget_manager;
extern crate diesel;

use self::diesel_budget_manager::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_budget_manager::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users.limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}: {}", user.id, user.email);
    }
}