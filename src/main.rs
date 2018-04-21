#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bcrypt;
extern crate chrono;

use bcrypt::{hash, verify};

use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::{self, Form, FlashMessage, Request, FromRequest};
use rocket::http::{Cookie, Cookies};
use rocket::Outcome;

use std::string::String;


mod lib;
use lib::models::category::Category;
use lib::models::expense::Expense;
use lib::controllers::usercontroller::{create_user, get_user_by_email, get_user_by_id};
use lib::controllers::categorycontroller::{create_category, get_categories_by_user_id, get_category_by_category_id, update_category, archive_category, unarchive_category, get_category_name_by_category_id};
use lib::controllers::expensecontroller::{create_expense, get_expenses_by_user_id, get_expense_by_expense_id, update_expense, delete_expense_by_id};

use lib::contexts::routecontexts::{RegisterContext, LoginContext, CategoryContext, ExpenseContext, StrCategories, StrExpenses, HomeContext, EditCategoryContext, UnauthorizedAccessContext, EditExpenseContext, DeleteExpenseContext, ReportContext};
use lib::forms::routeforms::{RegisterForm, LoginForm, CategoryForm, ExpenseForm, ExpenseDeleteForm};

use lib::utils::utilities::{not_logged_in, improper_user_access, flash_message_breakdown};

/*
INDEX GET AND POST
*/
#[get("/", rank = 1)]
fn index_get(_user_id_struct: IsUser) -> Redirect {
    return Redirect::to("/home");
}

#[get("/", rank = 2)]
fn index_get_nonuser() -> Redirect {
    return Redirect::to("/login");
}

/*
REGISTER GET AND POST
*/

#[get("/register", rank = 1)]
fn register_get(_user_id_struct: IsUser) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return improper_user_access("/home");
}

#[get("/register", rank = 2)]
fn register_get_nonuser(flash: Option<FlashMessage>) -> Template {
    //DONE: Make flash_message_breakdown function
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;
    
    // Create context that is passed to Tera Template
    let context = RegisterContext {
        title: String::from("Register"),
        authenticated: false,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
    };

    // Render our register Tera Template, and pass it the context
    Template::render("register", &context)
}

#[post("/register", rank = 1)]
fn register_post(_user_id_struct: IsUser) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return improper_user_access("/home");
}

#[post("/register", rank = 2, data = "<registerform>")]
fn register_post_nonuser(registerform: Form<RegisterForm>, mut cookies: Cookies) -> Result<Flash<Redirect>, Flash<Redirect>> {

    // Get the form in a Rust useable format
    let register_form = &registerform.get();

    // Get the user email from register_form
    let email_input = register_form.email.to_string();

    // Get the password from the register form
    let password_input = register_form.password.to_string();

    // Get the password confirmation from the regsiter form
    let password_confirm = register_form.confirm_password.to_string();

    // Create new mutable empty string in message to display to user
    //let mut messagelet mut message = String::new();
    
    // Check if the passwords match each other
    if &password_input == &password_confirm {

        let current_user = get_user_by_email(&email_input);
        if current_user.email != "" {
            //DONE: Return a Flash Redirect
            return Err(Flash::error(
                    Redirect::to("/register"),
                    format!("An account with email: {} already exists. Please login, or register with a new email address.", current_user.email)
                    ));
            //format!("{} already exists. Please login.", current_user.email));
            //message = format!("{} already exists. Please login.", current_user.email);
        }
        else {
            //Add user to DB
            let hashed = hash(&password_input.to_string(), 7);
            create_user(&email_input.to_string(), &hashed.expect("error"));
        
            //DONE: get user_id from the newly created user, and store it in a private_cookie
            let current_user = get_user_by_email(&email_input);
            cookies.add_private(Cookie::new("user_id", current_user.id.to_string()));

            //Create message indicating success
            //DONE: Return a Flash Redirect
            let msg = Flash::success(Redirect::to("/home"), format!("Account created for: {}", current_user.email));
            return Ok(msg)
            //message = format!("{} added as a new user", &email_input);
        }

        
    }
    else {
        //Create message indicating the passwords don't match
        //DONE: Return a Flash Redirect
        return Err(Flash::error(Redirect::to("/register"), "Passwords don't match"));
        //message = format!("PASSWORDS DON'T MATCH!");
    }

}

/*
LOGIN GET AND POST
*/
#[get("/login", rank = 1)]
fn login_get(_user_id_struct: IsUser) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return improper_user_access("/home");
}
//
#[get("/login", rank = 2)]
fn login_get_nonuser(flash: Option<FlashMessage>) -> Template {
    //DONE: Make flash_message_breakdown function
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;

    // Create context that is passed to Tera Template
    let context = LoginContext {
        title: String::from("Login"),
        authenticated: false,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
    };

    // Render our login Tera Template, and pass it the context
    Template::render("login", &context)
}

#[post("/login", rank = 1)]
fn login_post(_user_id_struct: IsUser) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return improper_user_access("/home");
}

#[post("/login", rank = 2, data = "<loginform>")]
fn login_post_nonuser(loginform: Form<LoginForm>, mut cookies: Cookies) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_form = &loginform.get();
    let email_input = login_form.email.to_string();
    //let password_input = hash(&login_form.password[..], 7).expect("error");
    let user = get_user_by_email(&email_input);
    let message: String;
    //DONE: Compare user, and password to DB records
    if user.email == "" {
        //user doesn't exist in DB
        message = String::from("Email, or Password incorrect");
        //DONE: Return a Flash Redirect
        return Err(Flash::error(Redirect::to("/login"), message));
    }
    else {
        //println!("user password: {}\nuser input:    {}\nlength: {}", user.password, password_input, user.password.len());
        if verify(&login_form.password, &user.password).expect("error") {
        //if user.password == password_input {
            //user exists, and password matches
            message = format!("{} logged in", user.email);
            cookies.remove_private(Cookie::named("user_id"));
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            //DONE: Return a Flash Redirect
            return Ok(Flash::success(Redirect::to("/home"), message));
        }
        else {
            //user exists, but password doesn't match
            message = String::from("Email, or Password incorrect");
            //DONE: Return a Flash Redirect
            return Err(Flash::error(Redirect::to("/login"), message));
        }
    }
}

/*
CATEGORY GET AND POST
*/
#[get("/category", rank = 1)]
fn category_get(str_user_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;

    //DONE: Get user categories and pass them as a vector to the CategoryContext
    let str_user_id = str_user_struct.0;

    //    .map(|c| format!("{}", c.value()))
    //    .unwrap_or_else(|| "-1".to_string());
    let int_user_id: i32 = str_user_id.parse().expect("Not a number");
    let user_categories: Vec<Category> = get_categories_by_user_id(&int_user_id);
    //let mut str_user_categories: Vec<String> = Vec::new();
    let num_of_categories = user_categories.len();
    let mut str_categories: Vec<StrCategories> = Vec::new();
    let mut archived_categories: Vec<StrCategories> = Vec::new();
    if user_categories.len() == 0 {
        //str_user_categories.push("No categories yet! Please add one.".to_string());
        //str_user_category_ids.push(format!("{}", "No categories found!"));
        //Do nothing!
    }
    else {
        for category in user_categories {
            if category.archived == false {
                str_categories.push(StrCategories{
                str_category_id: category.id,
                str_category_name: category.name,
                str_category_descrip: category.descrip,
                archived: category.archived,
                });
            }
            else {
                archived_categories.push(StrCategories{
                    str_category_id: category.id,
                    str_category_name: category.name,
                    str_category_descrip: category.descrip,
                    archived: category.archived,
                });
            }
        }
    }
    

    let context = CategoryContext {
        title: String::from("Categories"),
        authenticated: true,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
        total_categories: num_of_categories,
        str_categories: str_categories,
        archived_categories: archived_categories,
    };

    return Template::render("category", &context);

}

#[get("/category", rank = 2)]
fn category_get_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

#[post("/category", rank = 1, data = "<categoryform>")]
fn category_post(user_id_struct: IsUser, categoryform: Form<CategoryForm>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let category_form = &categoryform.get();
    let category_name = category_form.name.to_string();
    let category_descrip = category_form.descrip.to_string();

    if category_name == "".to_string() {
        return Err(Flash::error(Redirect::to("/category"), "Category name cannot be blank".to_string()))
    }
    else if category_descrip == "".to_string() {
        return Err(Flash::error(Redirect::to("/category"), "Category description cannot be blank".to_string()))
    }
    else {
        //DONE: get user id, create new Category in the database using user_id, category_name, & category_descrip
        let str_user_id = user_id_struct.0;
        //let str_user_id = get_user_id_from_cookies(cookies);
        let int_user_id: i32 = str_user_id.parse().expect("Not a number");
        create_category(&int_user_id, &category_name, &category_descrip);
        
        return Ok(Flash::success(Redirect::to("/category"), "Category successfully added".to_string()))
    }
}

#[post("/category", rank = 2)]
fn category_post_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

/*
EDIT CATEGORY GET
*/
#[get("/category/edit/<category_id>", rank = 1)]
fn category_edit_get(user_id_struct: IsUser, flash: Option<FlashMessage>, category_id: String) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Failed to parse");
    let int_category_id: i32 = category_id.parse().expect("Failed to parse");
    //DONE: Get category from category_id
    let category = get_category_by_category_id(&int_category_id);
    println!("User ID: {}", str_user_id);
    println!("Category User ID: {}", category.user_id);
    if category.user_id != int_user_id {
        /*
        The User id associated with this category, does not match the user_id with this authentaicted user
        */
        //DONE: compare category_user_id to str_user_id, if they don't match the user has requested access to a resource they don't have permission to view -- return the template, but the context should say they are not authorized to view
        //What if we just return an error template? Tried this, and it was real hokey -- decided to just return the same template, but the content changes on if they are authorized to view
        //I don't love that solution, I would prefer for the sever to redirect, so they don't land on the page -- but I am struggling with Rust/Rocket's strictly typed returns
        //In reading more about Rocket, you can return anythning that implements Responder, and I can attach Responder to a custom struct, so I could create my own return type, that allows for either a Flash<Redirect>, or a Template, but I'm not yet convinced that would be any better
        //println!("DON'T MATCH");
        
        let context = UnauthorizedAccessContext {
            title: String::from("Edit Category"),
            authenticated: true,
            authorized: false,
            flash_class: String::from(flash_class),
            flash_msg: String::from(flash_message),
        };
        return Template::render("category_edit", &context)
    }
    else {
        //DONE: compare category_user_id to str_user_id, if they do match, they are allowed to view, and edit this category, so return a template
        //println!("MATCH");
        /*
        The User id associated with this category, matches the user_id with this authenticated user
        */
        //DONE: create a category_edit context
        let context = EditCategoryContext {
            title: String::from("Edit Category"),
            authenticated: true,
            authorized: true,
            flash_class: String::from(flash_class),
            flash_msg: String::from(flash_message),
            total_categories: 1,
            category_id: category_id,
            category_name: category.name,
            category_descrip: category.descrip,
        };
        return Template::render("category_edit", &context);
    }
    
    
}

#[get("/category/edit/<category_id>", rank = 2)]
fn category_edit_get_nonuser(category_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _category_id = category_id;
    return not_logged_in("/login");
}

/*
EDIT CATEGORY POST
*/
#[post("/category/edit/<category_id>", rank = 1, data="<categoryform>")]
fn category_edit_post(user_id_struct: IsUser, category_id: String, categoryform: Form<CategoryForm>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let category_form = &categoryform.get();
    let category_name = category_form.name.to_string();
    let category_descrip = category_form.descrip.to_string();
    let int_category_id: i32 = category_id.parse().expect("unable to parse");
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("unable to parse");

    let category = get_category_by_category_id(&int_category_id);

    if category.user_id != int_user_id {
        return Err(Flash::error(Redirect::to("/category"), "You cannot edit a category that does not belong to you!".to_string()));
    }
    else {
        if category_name == "".to_string() {
            return Err(Flash::error(Redirect::to("/category"), "Category name cannot be blank".to_string()));
        }
        else if category_descrip == "".to_string() {
            return Err(Flash::error(Redirect::to("/category"), "Category description cannot be blank".to_string()))
        }
        else {
            //DONE: update Category in DB
            update_category(&int_category_id, &category_name, &category_descrip);
            return Ok(Flash::success(Redirect::to("/category"), "Category updated successfully!".to_string()));
        } 
    }

}

#[post("/category/edit/<category_id>", rank = 2)]
fn category_edit_post_nonuser(category_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _category_id = category_id;
    return not_logged_in("/login");
}

/*
CATEGORY ARCHIVE/UNARCHIVE
*/
#[get("/category/archive/<category_id>", rank = 1)]
fn category_archive_get(user_id_struct: IsUser, category_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Failed to parse");
    let int_category_id: i32 = category_id.parse().expect("Failed to parse");

    let category = get_category_by_category_id(&int_category_id);
    
    if category.user_id != int_user_id {
        return Err(Flash::error(Redirect::to("/category"), "You attempted to archive a category that is not associated with your account".to_string()));
    }
    else {
        archive_category(&int_category_id);
        return Ok(Flash::success(Redirect::to("/category"), "Category archived successfully"));
    }
}

#[get("/category/archive/<category_id>", rank = 2)]
fn category_archive_get_nonuser(category_id: String)  -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _category_id = category_id;
    return not_logged_in("/login");
}

#[get("/category/unarchive/<category_id>", rank = 1)]
fn category_unarchive_get(user_id_struct: IsUser, category_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Failed to parse");
    let int_category_id: i32 = category_id.parse().expect("Failed to parse");

    let category = get_category_by_category_id(&int_category_id);
    
    if category.user_id != int_user_id {
        return Err(Flash::error(Redirect::to("/category"), "You attempted to unarchive a category that is not associated with your account".to_string()));
    }
    else {
        unarchive_category(&int_category_id);
        return Ok(Flash::success(Redirect::to("/category"), "Category unarchived successfully"));
    }
}

#[get("/category/unarchive/<category_id>", rank = 2)]
fn category_unarchive_get_nonuser(category_id: String)  -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _category_id = category_id;
    return not_logged_in("/login");
}


/*
EXPENSE GET & POST
*/
#[get("/expense", rank = 1)]
fn expense_get(user_id_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;

    //DONE: Get Categories from user_id, so they can be passed to expense.html.tera
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Not a number!");
    let user_categories: Vec<Category> = get_categories_by_user_id(&int_user_id);
    let num_of_categories = user_categories.len();
    let mut str_categories: Vec<StrCategories> = Vec::new();

    if user_categories.len() == 0 {
        //Do nothing!
    }
    else {
        for category in user_categories {
            if category.archived == false {
                str_categories.push(StrCategories {
                str_category_id: category.id,
                str_category_name: category.name,
                str_category_descrip: category.descrip,
                archived: category.archived,
                });
            }
            
        }
    }

    //DONE: Get last five expenses from user_id
    //DONE: Get previous user expenses
    let user_expenses: Vec<Expense> = get_expenses_by_user_id(&int_user_id);
    let mut str_expenses: Vec<StrExpenses> = Vec::new();
    let num_of_expenses = user_expenses.len();
    let mut count = 0;
    if user_expenses.len() > 0 {
        for expense in user_expenses {
            //println!("Expense Created: {:?}", expense.created);
            if count > 4 {
                break;
            }
            let mut int_category_id : i32 = expense.category_id;
            str_expenses.push(StrExpenses {
                str_expense_id: expense.id,
                str_category_id: expense.category_id,
                str_category_name: get_category_name_by_category_id(&int_category_id),
                str_created: expense.created,
                str_name: expense.name,
                str_amount: expense.amount,
            });
            count = count + 1;
        }
    }


    let context = ExpenseContext {
        title: "Expense".to_string(),
        authenticated: true,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
        total_categories: num_of_categories,
        str_categories: str_categories,
        total_expenses: num_of_expenses,
        str_expenses: str_expenses,
    };
    return Template::render("expense", &context);
}

#[get("/expense", rank = 2)]
fn expense_get_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

//DONE: Create Expense Post request that implement IsUser guard
#[post("/expense", rank = 1, data = "<expenseform>")]
fn expense_post(user_id_struct: IsUser, expenseform: Form<ExpenseForm>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let expense_form = &expenseform.get();
    let category_id = expense_form.category_id.to_string();
    let expense_name = expense_form.name.to_string();
    let expense_amount = expense_form.amount.to_string();
    if expense_amount == "" {
        return Err(Flash::error(Redirect::to("/expense"), "Amount cannot be blank!".to_string()));
    }
    else {
        let float_expense_amount: f64 = expense_amount.parse().expect("Not a number");
            
        if float_expense_amount < 0.0 {
            return Err(Flash::error(Redirect::to("/expense"), "Amount cannot be less than 0!".to_string()));
        }
        else {
            //DONE: add create_expense function in the expense controller
            let str_user_id = user_id_struct.0;
            let int_user_id: i32 = str_user_id.parse().expect("Not a number");
            let int_category_id: i32 = category_id.parse().expect("Not a number");
            let str_expense_amount = float_expense_amount.to_string();
            create_expense(&int_user_id, &int_category_id, &expense_name, &str_expense_amount);
            return Ok(Flash::success(Redirect::to("/expense"), "Expense successfully added".to_string()));
        }
    }
}

#[post("/expense", rank = 2)]
fn expense_post_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

/*
EDIT EXPENSE GET
*/
#[get("/expense/edit/<expense_id>", rank = 1)]
fn expense_edit_get(user_id_struct: IsUser, flash: Option<FlashMessage>, expense_id: String) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Failed to parse");
    let int_expense_id: i32 = expense_id.parse().expect("Failed to parse");

    let user_categories: Vec<Category> = get_categories_by_user_id(&int_user_id);
    let num_of_categories = user_categories.len();
    let mut str_categories: Vec<StrCategories> = Vec::new();

    if user_categories.len() == 0 {
        //Do nothing!
    }
    else {
        for category in user_categories {
            if category.archived == false {
                str_categories.push(StrCategories {
                    str_category_id: category.id,
                    str_category_name: category.name,
                    str_category_descrip: category.descrip,
                    archived: category.archived,
                });
            }
        }
    }

    let expense = get_expense_by_expense_id(&int_expense_id);

    if expense.user_id != int_user_id {
        /*
        The user id associated with this expense does not match the user_id of this authenticated user
        */
        let context = UnauthorizedAccessContext {
            title: String::from("Edit Expense"),
            authenticated: true,
            authorized: false,
            flash_class: String::from(flash_class),
            flash_msg: String::from(flash_message),
        };
        return Template::render("expense_edit", &context);
    }
    else {
        let context = EditExpenseContext {
            title: String::from("Edit Expense"),
            authenticated: true,
            authorized: true,
            flash_class: String::from(flash_class),
            flash_msg: String::from(flash_message),
            total_categories: num_of_categories,
            str_categories: str_categories,
            expense_id: expense.id.to_string(),
            category_id: expense.category_id.to_string(),
            expense_name: expense.name,
            expense_amount: expense.amount,

        };
        return Template::render("expense_edit", &context);
    }
}

#[get("/expense/edit/<expense_id>", rank = 2)]
fn expense_edit_get_nonuser(expense_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _expense_id = expense_id;
    return not_logged_in("/login");
}

/*
EDIT EXPENSE POST
*/
#[post("/expense/edit/<expense_id>", rank = 1, data="<expenseform>")]
fn expense_edit_post(user_id_struct: IsUser, expense_id: String, expenseform: Form<ExpenseForm>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let expense_form = &expenseform.get();
    let category_id = expense_form.category_id.to_string();
    let expense_name = expense_form.name.to_string();
    let expense_amount = expense_form.amount.to_string();
    let int_expense_id: i32 = expense_id.parse().expect("unable to parse");
    let int_category_id: i32 = category_id.parse().expect("unable to parse");
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("unable to parse");

    let expense = get_expense_by_expense_id(&int_expense_id);

    if expense.user_id != int_user_id {
        return Err(Flash::error(Redirect::to("/expense"), "You cannot edit an expense that does not belong to you!".to_string()));
    }
    else {
        if expense_name == "".to_string() {
            return Err(Flash::error(Redirect::to("/expense"), "Expense name cannot be blank".to_string()));
        }
        else if expense_amount == "".to_string() {
            return Err(Flash::error(Redirect::to("/expense"), "Expense amount cannot be blank".to_string()));
        }
        else {
            //TODO: update expense in DB
            update_expense(&int_expense_id, &int_category_id, &expense_name, &expense_amount);
            return Ok(Flash::success(Redirect::to("/expense"), "Expense updated successfully!".to_string()));
        }
    }
}

#[post("/expense/edit/<expense_id>", rank = 2)]
fn expense_edit_post_nonuser(expense_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _expense_id = expense_id;
    return not_logged_in("/login");
}

/*
EXPENSE DELETE GET
*/
#[get("/expense/delete/<expense_id>", rank = 1)]
fn expense_delete_get(user_id_struct: IsUser, expense_id: String, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Failed to parse");
    let int_expense_id: i32 = expense_id.parse().expect("Failed to parse");

    let expense = get_expense_by_expense_id(&int_expense_id);

    if expense.user_id != int_user_id {
        let context = UnauthorizedAccessContext {
            title: String::from("Edit Category"),
            authenticated: true,
            authorized: false,
            flash_class: String::from(flash_class),
            flash_msg: String::from(flash_message),
        };
        return Template::render("expense_delete", &context);
    }
    else {
        let context = DeleteExpenseContext {
            title: String::from("Delete Expense"),
            authenticated: true,
            authorized: true,
            flash_class: String::from(flash_class),
            flash_msg: String::from(flash_message),
            expense_id: int_expense_id,
        };
        return Template::render("expense_delete", &context);
    }
}

#[get("/expense/delete/<expense_id>", rank = 2)]
fn expense_delete_get_nonuser(expense_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let _expense_id = expense_id;
    return not_logged_in("/login");
}

/*
EXPENSE DELETE POST
*/
//ExpenseDeleteForm
#[post("/expense/delete/<expense_id>", rank = 1, data = "<expensedeleteform>")]
fn expense_delete_post(user_id_struct: IsUser, expense_id: String, expensedeleteform: Form<ExpenseDeleteForm>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("failed to parse");
    let int_expense_id: i32 = expense_id.parse().expect("failed to parse");

    let expense_delete_form = &expensedeleteform.get();
    let expense_id = expense_delete_form.delete_expense_id.to_string();

    let expense = get_expense_by_expense_id(&int_expense_id);
    println!("Expense.user_id: {}", expense.user_id.to_string());
    println!("expense_id: {}", expense_id);
    if expense.user_id != int_user_id {
        return Err(Flash::error(Redirect::to("/expense"), "You attempted to delete an expense that is not associated with your account!".to_string()));
    }
    else {
        if expense.id.to_string() != expense_id {
            return Err(Flash::error(Redirect::to("/expense"), "Expense IDs do not match!"));
        }
        else {
            let _deleted = delete_expense_by_id(&int_expense_id);
            return Ok(Flash::success(Redirect::to("/expense"), "Expense successfully deleted!".to_string()));
        }
        
    }
}

#[post("/expense/delete/<expense_id>", rank = 2)]
fn expense_delete_post_nonuser(expense_id: String) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

/*
REPORTS GET
*/
#[get("/reports", rank = 1)]
fn reports_get(user_id_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;
    let str_user_id = user_id_struct.0;
    let int_user_id: i32 = str_user_id.parse().expect("Failed to parse");

    let mut str_categories = Vec::new();
    let categories = get_categories_by_user_id(&int_user_id);

    let total_categories = categories.len();
    
    for category in categories {
        str_categories.push(StrCategories {
            str_category_id: category.id,
            str_category_name: category.name,
            str_category_descrip: category.descrip,
            archived: category.archived,
        });
    }
    
    
    let mut str_expenses = Vec::new();
    let mut total_expense_amount: f32 = 0.0;
    let expenses = get_expenses_by_user_id(&int_user_id);

    let total_expenses = expenses.len();

    for expense in expenses {
        let mut int_category_id : i32 = expense.category_id;
        let new_amount: f32 = expense.amount.parse().expect("cound not parse");
        total_expense_amount = total_expense_amount + new_amount;
        str_expenses.push(StrExpenses {
            str_expense_id: expense.id,
            str_category_id: expense.category_id,
            str_category_name: get_category_name_by_category_id(&int_category_id),
            str_created: expense.created,
            str_name: expense.name,
            str_amount: expense.amount,
        });
    }
    

    let context = ReportContext {
        title: String::from("Reports"),
        authenticated: true,
        authorized: true,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
        str_categories: str_categories,
        total_categories: total_categories,
        str_expenses: str_expenses,
        total_expenses: total_expenses,
        total_expense_amount: total_expense_amount,

    };

    return Template::render("reports", &context);
}

#[get("/reports", rank = 2)]
fn reports_get_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

/*
LOGOUT GET
*/
#[get("/logout", rank = 1)]
fn logout_get(_user_id_struct: IsUser, mut cookies: Cookies) -> Result<Flash<Redirect>, Flash<Redirect>> {
    cookies.remove_private(Cookie::named("user_id"));
    return Ok(Flash::success(Redirect::to("/login"), "Successfully logged out".to_string()));
}

#[get("/logout", rank = 2)]
fn logout_get_nonuser() -> Redirect {
    return Redirect::to("/login");
}

/*
WELCOME/HOME GET
*/
#[get("/home", rank = 1)]
fn home_get(user_id_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_class = message.0;

    let int_user_id: i32 = user_id_struct.0.parse().expect("Not a number");
    let current_user = get_user_by_id(&int_user_id);

    let context = HomeContext {
        title: "Home".to_string(),
        authenticated: true,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
        user_email: current_user.email,
    };

    return Template::render("home", &context);
}

#[get("/home", rank = 2)]
fn home_get_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

struct IsUser(String);

impl<'a, 'r> FromRequest<'a, 'r> for IsUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<IsUser, ()> {
        if request.cookies().get_private("user_id").is_none() {
            return Outcome::Forward(());
        }
        else {
            let str_user_id = request.cookies().get_private("user_id")
            .map(|c| format!("{}", c.value()))
            .unwrap_or_else(|| "-1".to_string());
            return Outcome::Success(IsUser(str_user_id))
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        index_get,
        index_get_nonuser,
        register_get,
        register_get_nonuser,
        register_post,
        register_post_nonuser,
        login_get,
        login_get_nonuser,
        login_post,
        login_post_nonuser,
        logout_get,
        logout_get_nonuser,
        home_get,
        home_get_nonuser,
        category_get,
        category_get_nonuser,
        category_post,
        category_post_nonuser,
        category_edit_get,
        category_edit_get_nonuser,
        category_edit_post,
        category_edit_post_nonuser,
        category_archive_get,
        category_archive_get_nonuser,
        category_unarchive_get,
        category_unarchive_get_nonuser,
        expense_get,
        expense_get_nonuser,
        expense_post,
        expense_post_nonuser,
        expense_edit_get,
        expense_edit_get_nonuser,
        expense_edit_post,
        expense_edit_post_nonuser,
        expense_delete_get,
        expense_delete_get_nonuser,
        expense_delete_post,
        expense_delete_post_nonuser,
        reports_get,
        reports_get_nonuser,
    ])
    .attach(Template::fairing())
}


fn main() {
    rocket().launch();
}
