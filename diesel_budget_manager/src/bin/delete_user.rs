extern crate diesel_budget_manager;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_budget_manager::*;
use std::env::args;

fn main() {
    use diesel_budget_manager::schema::users::dsl::*;

    /* 
    Currently we delete the user by getting the email as an argument
    being passed in at run time -- a great Proof of Concept
    but will need to be changed for the web app

    TODO: We will get the email from the session and an additional confirmation
    */

    let target = args().nth(1)
        .expect("Expected a target");
    
    let pattern = format!("%{}%", target);

    let connection = establish_connection();

    let num_deleted = diesel::delete(users.filter(email.like(pattern)))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deteled user that matches this email: {}", target);
}