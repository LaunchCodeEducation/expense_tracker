use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::insert_into;
use dotenv::dotenv;
use std::env;

//extern crate schema;
//extern crate models;

use super::models::user::{User, NewUser};

//use models::user::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, password: &'a str) -> User {
    use lib::schema::users;

    let new_user = NewUser {
        email: email,
        password: password,
    };

    insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_user_by_email<'a>(input_email: &'a str) -> User {
    use lib::schema::users::dsl::*;

    //This function returns a User struct based on an email address
    //It also returns an emtpy User object if it's not found

    let mut user:User = User {
        id: -1,
        email: String::from(""),
        password: String::from("")
    };

    let conn = establish_connection();

    if users.filter(email.eq(input_email))
        .first::<User>(&conn)
        .is_ok() {
        //println!("established connection, before user");
        let user = users.filter(email.eq(input_email))
            .first::<User>(&conn)
            .expect("Error loading user");
        //println!("after user");
        return user;
    }
    else {
        
        User {
            id: -1,
            email: String::from(""),
            password: String::from("")
        }
    }

    

    
}

pub fn get_user_by_id<'a>(input_id: &'a i32) -> User {
    use lib::schema::users::dsl::*;

    let mut user: User = User {
        id: -1,
        email: String::from(""),
        password: String::from("")
    };

    let conn = establish_connection();

    if users.filter(id.eq(input_id)) 
        .first::<User>(&conn)
        .is_ok() {

        let user = users.filter(id.eq(input_id))
            .first::<User>(&conn)
            .expect("Error loading user");
        
        return user;
    }
    else {
        User {
            id: -1,
            email: String::from(""),
            password: String::from("")
        }
    }

}