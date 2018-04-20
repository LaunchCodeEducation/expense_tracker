use diesel::prelude::*;
use diesel::{insert_into, update, delete};

use lib::models::expense::{Expense, NewExpense};
use lib::db_manager::establish_connection;
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
            .order(id.desc())
            .get_results::<Expense>(&conn)
            .expect("Error loading expense table");

        return user_expenses;
    }
    else {
        let user_expenses: Vec<Expense> = Vec::new();
        return user_expenses;
    }

}

pub fn get_expense_by_expense_id<'a>(input_id: &'a i32) -> Expense {
    use lib::schema::expenses::dsl::*;

    let conn = establish_connection();

    if expenses.filter(id.eq(input_id))
        .first::<Expense>(&conn)
        .is_ok() {

        let expense = expenses.filter(id.eq(input_id))
            .first::<Expense>(&conn)
            .expect("Error loading expense");

        return expense;
    }
    else {
        Expense {
            id: -1,
            user_id: -1,
            category_id: -1,
            created: Utc::now().to_string(),
            name: String::from(""),
            amount: String::from("")

        }
    }
}

pub fn update_expense<'a>(expense_id: &'a i32, cat_id: &'a i32, n: &'a str, a: &'a str) -> Expense {
    use lib::schema::expenses::columns::{category_id, name, amount};
    use lib::schema::expenses::dsl::*;

    let conn = establish_connection();

    let target = expenses.filter(id.eq(expense_id));

    update(target)
        .set((
            category_id.eq(cat_id),
            name.eq(n),
            amount.eq(a)
        ))
        .get_result(&conn)
        .expect("error updating expense")
}

pub fn delete_expense_by_id<'a>(expense_id: &'a i32) -> bool {
    use lib::schema::expenses::columns::id;
    use lib::schema::expenses::dsl::*;

    let conn = establish_connection();

    let target = expenses.filter(id.eq(expense_id));

    let num_deleted = delete(target)
        .execute(&conn)
        .expect("error deleting expense");

    if num_deleted > 0 {
        return true;
    }
    else {
        return false;
    }
}