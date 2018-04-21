use diesel::prelude::*;
use diesel::{insert_into, update};

use lib::models::category::{Category, NewCategory};
use lib::db_manager::establish_connection;

pub fn create_category<'a>(user_id: &'a i32, name: &'a str, descrip: &'a str) -> Category {
    use lib::schema::categories;

    let conn = establish_connection();

    let new_category = NewCategory {
        user_id: user_id,
        name: name,
        descrip: descrip,
        archived: false,
    };

    insert_into(categories::table)
        .values(&new_category)
        .get_result(&conn)
        .expect("Error saving category")
}

pub fn update_category<'a>(category_id: &'a i32, n: &'a str, d: &'a str) -> Category {
    use lib::schema::categories::columns::id;
    use lib::schema::categories::columns::name;
    use lib::schema::categories::columns::descrip;
    use lib::schema::categories::dsl::*;

    let conn = establish_connection();

    let target = categories.filter(id.eq(category_id));

    update(target)
        .set((
            name.eq(n),
            descrip.eq(d)
        ))
        .get_result(&conn)
        .expect("error updating category")
}

//DONE: archive_category function it should be very similar to update_category, but only changes the archived column
pub fn archive_category(category_id: &i32) -> Category {
    use lib::schema::categories::columns::id;
    use lib::schema::categories::columns::archived;
    use lib::schema::categories::dsl::*;

    let conn = establish_connection();

    let target = categories.filter(id.eq(category_id));

    update(target)
        .set(archived.eq(true))
        .get_result(&conn)
        .expect("error archiving category")
}

//DONE: unarchive_category function it should be very similar to arhive_category
pub fn unarchive_category(category_id: &i32) -> Category {
    use lib::schema::categories::columns::id;
    use lib::schema::categories::columns::archived;
    use lib::schema::categories::dsl::*;

    let conn = establish_connection();

    let target = categories.filter(id.eq(category_id));

    update(target)
        .set(archived.eq(false))
        .get_result(&conn)
        .expect("error archiving category")
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
            descrip: String::from(""),
            archived: false
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
            .order(id)
            .get_results::<Category>(&conn)
            .expect("Error loading tables");

        return user_categories;
        
    }
    else {
        let user_categories: Vec<Category> = Vec::new();
        return user_categories;
    }
}

pub fn get_category_name_by_category_id<'a>(input_id: &'a i32) -> String {
    let category = get_category_by_category_id(&input_id);
    return category.name;
}