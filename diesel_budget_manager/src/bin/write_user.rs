extern crate diesel_budget_manager;
extern crate diesel;

use self::diesel_budget_manager::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Enter email: ");
    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = &email[..(email.len() - 1)]; // this drops the newline in all read_line statements
    println!("Enter password: ");
    let mut password = String::new();
    stdin().read_to_string(&mut password).unwrap();
    let password = &password[..(password.len() - 1)];

    let user = create_user(&connection, email, password);
    println!("\nSaved user {} with id {}", user.email, user.id);
}

const EOF: &'static str = "CTRL+D";