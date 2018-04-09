use diesel::prelude::*;
use diesel::insert_into;

use lib::models::expense::{Expense, NewExpense};
use lib::db_manager::establish_connection;

pub fn create_expense<'a>(user_id: &'a i32, category_id: &'a i32, name: &'a str, amount: &'a str) -> Expense {
    use lib::schema::expenses;

    let conn = establish_connection();

    let new_expense = NewExpense {
        user_id: user_id,
        category_id: category_id,
        name: name,
        amount: amount,
    };

    insert_into(expenses::table)
        .values(&new_expense)
        .get_result(&conn)
        .expect("Error saving expense")
}