use lib::schema::categories;

#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub descrip: String,
    pub archived: bool,
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategory<'a> {
    pub user_id: &'a i32,
    pub name: &'a str,
    pub descrip: &'a str,
    pub archived: bool,

}