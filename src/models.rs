use schema::users;
use schema::categories;
use schema::expenses;

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