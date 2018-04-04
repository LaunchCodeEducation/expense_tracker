use diesel::prelude::*;
use diesel::insert_into;

use lib::models::category::{Category, NewCategory};
use lib::db_manager::establish_connection;

pub fn create_category<'a>(user_id: &'a i32, name: &'a str, descrip: &'a str) -> Category {
    use lib::schema::categories;

    let conn = establish_connection();

    let new_category = NewCategory {
        user_id: user_id,
        name: name,
        descrip: descrip,
    };

    insert_into(categories::table)
        .values(&new_category)
        .get_result(&conn)
        .expect("Error saving category")
}

pub fn get_category_by_category_id<'a>(input_id: &'a i32) -> Category {
    use lib::schema::categories::dsl::*;

    let conn = establish_connection();

    if categories.filter(id.eq(input_id))
        .first::<Category>(&conn)
        .is_ok() {

        let category = categories.filter(id.eq(input_id))
            .first::<Category>(&conn)
            .expect("Error loading category");

        return category
    }
    else {
        Category {
            id: -1,
            user_id: -1,
            name: String::from(""),
            descrip: String::from("")
        }
    }
}

pub fn get_categories_by_user_id<'a>(input_id: &'a i32) -> Vec<Category> {
    use lib::schema::categories::dsl::*;

    let conn = establish_connection();

    if categories.filter(user_id.eq(input_id))
        .get_results::<Category>(&conn)
        .is_ok() {
        
        let user_categories: Vec<Category> = categories.filter(user_id.eq(input_id))
            .get_results::<Category>(&conn)
            .expect("Error loading tables");

        return user_categories;
        
    }
    else {
        let user_categories: Vec<Category> = Vec::new();
        return user_categories;
    }


    //let mut user_categories: Vec<Category> = Vec::new();

}