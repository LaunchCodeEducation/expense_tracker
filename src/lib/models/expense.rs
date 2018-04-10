use lib::schema::expenses;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Expense {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub created: SystemTime,
    pub name: String,
    pub amount: String,
}

#[derive(Insertable)]
#[table_name="expenses"]
pub struct NewExpense<'a> {
    pub user_id: &'a i32,
    pub category_id: &'a i32,
    pub created: SystemTime,
    pub name: &'a str,
    pub amount: &'a str,
}