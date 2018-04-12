use diesel::prelude::*;
use diesel::insert_into;

use lib::models::expense::{Expense, NewExpense};
use lib::db_manager::establish_connection;
use std::time::SystemTime;
use chrono::prelude::Utc;

pub fn create_expense<'a>(user_id: &'a i32, category_id: &'a i32, name: &'a str, amount: &'a str) -> Expense {
    use lib::schema::expenses;

    let conn = establish_connection();

    let new_expense = NewExpense {
        user_id: user_id,
        category_id: category_id,
        created: &Utc::now().to_string(),
        name: name,
        amount: amount,
    };

    insert_into(expenses::table)
        .values(&new_expense)
        .get_result(&conn)
        .expect("Error saving expense")
}

pub fn get_expenses_by_user_id<'a>(input_id: &'a i32) -> Vec<Expense> {
    use lib::schema::expenses::dsl::*;

    let conn = establish_connection();

    if expenses.filter(user_id.eq(input_id))
        .get_results::<Expense>(&conn)
        .is_ok() {

        let user_expenses: Vec<Expense> = expenses.filter(user_id.eq(input_id))
            .get_results::<Expense>(&conn)
            .expect("Error loading expense table");

        return user_expenses;
    }
    else {
        let user_expenses: Vec<Expense> = Vec::new();
        return user_expenses;
    }

}