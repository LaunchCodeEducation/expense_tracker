/* CONTEXTS
Contexts are Rocket's way of passing information to the HTML doc
So when we want to pass information to our Tera template, we have to create
a new context that defines what that data will be.

Since I pass information to almost every route, there needs to be a context created
for each GET route.
*/

#[derive(Serialize)]
pub struct RegisterContext {
    pub title: String,
    pub authenticated: bool,
    pub flash_class: String,
    pub flash_msg: String,
}

#[derive(Serialize)]
pub struct LoginContext {
    pub title: String,
    pub authenticated: bool,
    pub flash_class: String,
    pub flash_msg: String,
}

#[derive(Serialize)]
pub struct CategoryContext {
    pub title: String,
    pub authenticated: bool,
    pub flash_class: String,
    pub flash_msg: String,
    //DONE: Add categories as a vector of Category objects
    pub total_categories: usize,
    pub str_categories: Vec<StrCategories>,
}

#[derive(Serialize)]
pub struct EditCategoryContext {
    pub title: String,
    pub authenticated: bool,
    pub authorized: bool,
    pub flash_class: String,
    pub flash_msg: String,
    pub total_categories: usize,
    pub category_id: String,
    pub category_name: String,
    pub category_descrip: String,
    //pub str_categories: Vec<StrCategories>,
}

#[derive(Serialize)]
pub struct StrCategories {
    pub str_category_id: i32,
    pub str_category_name: String,
    pub str_category_descrip: String,
}

#[derive(Serialize)]
pub struct ExpenseContext {
    pub title: String,
    pub authenticated: bool,
    pub flash_class: String,
    pub flash_msg: String,
    pub total_categories: usize,
    pub str_categories: Vec<StrCategories>,
    pub total_expenses: usize,
    //TODO: Add last 5 expenses as a vector of Expense objects
    pub str_expenses: Vec<StrExpenses>,
}

#[derive(Serialize)]
pub struct StrExpenses {
    pub str_expense_id: i32,
    pub str_category_id: i32,
    pub str_created: String,
    pub str_name: String,
    pub str_amount: String,
}

#[derive(Serialize)]
pub struct HomeContext {
    pub title: String,
    pub authenticated: bool,
    pub flash_class: String,
    pub flash_msg: String,
    pub user_email: String,
}

#[derive(Serialize)]
pub struct UnauthorizedAccessContext {
    pub title: String,
    pub authenticated: bool,
    pub authorized: bool,
    pub flash_class: String,
    pub flash_msg: String,
}