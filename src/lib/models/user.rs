use lib::schema::users;

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