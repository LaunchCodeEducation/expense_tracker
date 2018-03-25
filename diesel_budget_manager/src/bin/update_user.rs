extern crate diesel_budget_manager;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_budget_manager::*;
use self::models::User;
use std::env::args;

fn main() {
    use diesel_budget_manager::schema::users::dsl::{users, password};

    /* 
    Currently we change the password by getting it as an argument
    being passed in at run time -- a great Proof of Concept
    but will need to be changed for the web app

    TODO: We will get the id from the session and the new password from a form
    */
    let id = args().nth(1)
        .expect("update_user requires a user id")
        .parse::<i32>().expect("Invalid ID");
    
    let connection = establish_connection();

    let user = diesel::update(users.find(id))
        .set(password.eq("changed_password"))
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user {}", id));

    println!("Changed user password of: {}", user.email);       
}