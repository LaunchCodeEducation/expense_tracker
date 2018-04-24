use diesel::prelude::*;
use diesel::{insert_into, update};

use lib::models::user::{User, NewUser};
use lib::db_manager::establish_connection;

//CREATE USER
pub fn create_user<'a>(email: &'a str, password: &'a str) -> User {
    use lib::schema::users;

    let conn = establish_connection();

    let new_user = NewUser {
        email: email,
        password: password,
    };

    insert_into(users::table)
        .values(&new_user)
        .get_result(&conn)
        .expect("Error saving user")
}

//READ USER (by email)
pub fn get_user_by_email<'a>(input_email: &'a str) -> User {
    use lib::schema::users::dsl::*;

    //This function returns a User struct based on an email address
    //It also returns an emtpy User object if it's not found

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

//READ USER (by id)
pub fn get_user_by_id<'a>(input_id: &'a i32) -> User {
    use lib::schema::users::dsl::*;

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

pub fn update_user_email<'a>(input_id: &'a i32, new_email: &'a str) -> User {
    use lib::schema::users::columns::email;
    use lib::schema::users::dsl::*;

    let conn = establish_connection();

    let target = users.filter(id.eq(input_id));

    update(target)
        .set(
            email.eq(new_email)
        )
        .get_result(&conn)
        .expect("Error updating user")
}
