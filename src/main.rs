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

use bcrypt::{hash, verify};

use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::{self, Form, FlashMessage, Request, FromRequest};
use rocket::http::{Cookie, Cookies};
use rocket::Outcome;


use std::string::String;

mod lib;
use lib::models::category::Category;
use lib::controllers::usercontroller::{create_user, get_user_by_email, get_user_by_id};
use lib::controllers::categorycontroller::{create_category, get_categories_by_user_id};
use lib::controllers::expensecontroller::{create_expense};

//GET USER ID FROM cookies

/*
CONTEXTS
Contexts are Rocket's way of passing information to the HTML doc
So when we want to pass information to our Tera template, we have to create
a new context that defines what that data will be.

Since I pass information to almost every route, there needs to be a context created
for each GET route.

FORMS
Forms are Rocket's way of passing information from a POST request
to the server (here). 

Since Rust is statically typed, and compiled
we have to define a Form for each POST route.
*/

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
REGISTER CONTEXTS & FORMS
*/
#[derive(Serialize)]
struct RegisterContext {
    title: String,
    authenticated: bool,
    flash_class: String,
    flash_msg: String,
}

#[derive(FromForm)]
struct RegisterForm {
    email: String,
    password: String,
    confirm_password: String,
}

fn not_logged_in(route: &str) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return Err(Flash::error(Redirect::to(route), "You need to login, or register!"));
}

fn improper_user_access(route: &str) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return Err(Flash::error(Redirect::to(route), "You are already logged in! Logout to login, or create a new account."));
}

//The following function is deprecated now that this project uses a request guard for users
fn logged_in(mut cookies: Cookies) -> bool {
    //DONE: Remove this function, and simply get the user_id_from_cookies each function and if the user_id is -1, the user isn't logged in
    //DONE: Every GET, and POST should check if the user is logged in, and if not redirect them to the login page, right now you can kind of squeak past this by already having a cookie in memory, or hard coding a URL in to the address bar
    //let cooks = cookies.get_private("user_id").is_none();
    if cookies.get_private("user_id").is_none() {
        return false
    }
    else {
        return true;
    }
    
}

//The following funciton is deprecated now that all GETS and POSTS are authenticated, and the user_id is passed as a struct
fn get_user_id_from_cookies(mut cookies: Cookies) -> String {
    if cookies.get_private("user_id").is_none() {
        return "-1".to_string();
    }
    else {
        let cooks = cookies.get_private("user_id")
            .map(|c| format!("{}", c.value()))
            .unwrap_or_else(|| "-1".to_string());

        return cooks.to_string();
    }
    
}

/*
REGISTER GET AND POST
*/

fn flash_message_breakdown(flash: Option<FlashMessage>) -> (String, String) {
    // Access the flash message result so it can be added to the context
    let flash_message: String;
    // Unwrap result or else return a string that looks like: "no class&no flash message"
    flash_message = flash.map(|msg| format!("{}&{}", msg.name().to_string(), msg.msg().to_string()))
        .unwrap_or_else(|| "no class&No flash message".to_string());
    // Split the flash message into a flash message like: "User logged in" and a flash class like: "success"
    let string_split_position = flash_message.find('&');
    let flash_message_split = flash_message.split_at(string_split_position.unwrap_or_else(|| 0));

    let message = (flash_message_split.0.to_string(), flash_message_split.1.to_string());
    return message;
}

#[get("/register", rank = 1)]
fn register_get(_user_id_struct: IsUser) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return improper_user_access("/home");
}

#[get("/register", rank = 2)]
fn register_get_nonuser(flash: Option<FlashMessage>) -> Template {
    //DONE: Make flash_message_breakdown function
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    // Depending on the flash class convert the flash_class into CSS readable code that can be passed to our Tera template
    let flash_class: String;
    if flash_type == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }
    
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
LOGIN CONTEXT & FORMS
*/
#[derive(Serialize)]
struct LoginContext {
    title: String,
    authenticated: bool,
    flash_class: String,
    flash_msg: String,
}

#[derive(FromForm)]
struct LoginForm {
    email: String,
    password: String,
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
    let flash_type = message.0;

    // Depending on the flash class convert the flash_class into CSS readable code that can be passed to our Tera template
    let flash_class: String;
    if flash_type == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }

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
CATEGORY CONTEXT & FORMS
*/
#[derive(Serialize)]
struct CategoryContext {
    title: String,
    authenticated: bool,
    flash_class: String,
    flash_msg: String,
    //DONE: Add categories as a vector of Category objects
    total_categories: usize,
    str_categories: Vec<StrCategories>,
}

#[derive(FromForm)]
struct CategoryForm {
    name: String,
    descrip: String,
}

#[derive(Serialize)]
struct StrCategories {
    str_category_id: i32,
    str_category_name: String,
    str_category_descrip: String,
}

/*
CATEGORY GET AND POST
*/
#[get("/category", rank = 1)]
fn category_get(str_user_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    // Depending on the flash class convert the flash_class into CSS readable code that can be passed to our Tera template
    let flash_class: String;
    if flash_type == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }

    //DONE: Get user categories and pass them as a vector to the CategoryContext
    let str_user_id = str_user_struct.0;

    //    .map(|c| format!("{}", c.value()))
    //    .unwrap_or_else(|| "-1".to_string());
    let int_user_id: i32 = str_user_id.parse().expect("Not a number");
    let user_categories: Vec<Category> = get_categories_by_user_id(&int_user_id);
    //let mut str_user_categories: Vec<String> = Vec::new();
    let num_of_categories = user_categories.len();
    let mut str_categories: Vec<StrCategories> = Vec::new();
    if user_categories.len() == 0 {
        //str_user_categories.push("No categories yet! Please add one.".to_string());
        //str_user_category_ids.push(format!("{}", "No categories found!"));
        //Do nothing!
    }
    else {
        for category in user_categories {
            str_categories.push(StrCategories{
                str_category_id: category.id,
                str_category_name: category.name,
                str_category_descrip: category.descrip,
            });

        }
    }
    

    let context = CategoryContext {
        title: String::from("Categories"),
        authenticated: true,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
        total_categories: num_of_categories,
        str_categories: str_categories,
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
EXPENSE CONTEXT & FORM
*/

#[derive(Serialize)]
struct ExpenseContext {
    title: String,
    authenticated: bool,
    flash_class: String,
    flash_msg: String,
    total_categories: usize,
    str_categories: Vec<StrCategories>,
    //TODO: Add last 5 expenses as a vector of Expense objects
}

//DONE: Create Expense Form
#[derive(FromForm)]
struct ExpenseForm {
    category_id: String,
    name: String,
    amount: String,
}



/*
EXPENSE GET & POST
*/
#[get("/expense", rank = 1)]
fn expense_get(user_id_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    // Depending on the flash class convert the flash_class into CSS readable code that can be passed to our Tera template
    let flash_class: String;
    if flash_type == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }

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
            str_categories.push(StrCategories {
                str_category_id: category.id,
                str_category_name: category.name,
                str_category_descrip: category.descrip,
            });
        }
    }

    //TODO: Get last five expenses from user_id

    let context = ExpenseContext {
        title: "Expense".to_string(),
        authenticated: true,
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
        total_categories: num_of_categories,
        str_categories: str_categories,
    };
    return Template::render("expense", &context);
}

#[get("/expense", rank = 2)]
fn expense_get_nonuser() -> Result<Flash<Redirect>, Flash<Redirect>> {
    return not_logged_in("/login");
}

//TODO: Create Expense Post request that implement IsUser guard
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
            //TODO: add create_expense function in the expense controller
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
WELCOME/HOME CONTEXT
*/
#[derive(Serialize)]
struct HomeContext {
    title: String,
    authenticated: bool,
    flash_class: String,
    flash_msg: String,
    user_email: String,
}

/*
WELCOME/HOME GET
*/
#[get("/home", rank = 1)]
fn home_get(user_id_struct: IsUser, flash: Option<FlashMessage>) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    let int_user_id: i32 = user_id_struct.0.parse().expect("Not a number");
    let current_user = get_user_by_id(&int_user_id);

    let context = HomeContext {
        title: "Home".to_string(),
        authenticated: true,
        flash_class: flash_type.to_string(),
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
        expense_get,
        expense_get_nonuser,
        expense_post,
        expense_post_nonuser,
    ])
    .attach(Template::fairing())
}


fn main() {
    rocket().launch();
}
