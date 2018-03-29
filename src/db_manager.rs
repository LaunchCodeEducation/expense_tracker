//#[macro_use]
//extern crate diesel;
//extern crate dotenv;

//pub mod db_manager::schema;
//pub mod self::models;

mod schema {
    table! {
    categories (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        descrip -> Varchar,
    }
}

table! {
    expenses (id) {
        id -> Int4,
        user_id -> Int4,
        category_id -> Int4,
        amount -> Numeric,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(categories -> users (user_id));
joinable!(expenses -> categories (category_id));
joinable!(expenses -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    expenses,
    users,
);
}

pub mod models {
    use db_manager::schema::users;
    use db_manager::schema::categories;
    use db_manager::schema::expenses;

    #[derive(Queryable)]
    pub struct User {
        pub id: i32,
        pub email: String,
        pub password: String,
    }

    #[derive(Insertable)]
    #[table_name="users"]
    pub struct NewUser<'a> {
        pub email: &'a str,
        pub password: &'a str,
    }

    pub struct Category {
        pub id: i32,
        pub user_id: i32,
        pub name: String,
        pub descrip: String,
    }

    pub struct Expense {
        pub id: i32,
        pub user_id: i32,
        pub category_id: i32,
        pub amount: f32,
    }
}

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use db_manager::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, password: &'a str) -> User {
    use db_manager::schema::users;

    let new_user = NewUser {
        email: email,
        password: password,
    };

    super::diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_user_by_email<'a>(input_email: &'a str) -> User {
    use db_manager::schema::users::dsl::*;

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
    use db_manager::schema::users::dsl::*;

    let mut user:User = User {
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