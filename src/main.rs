#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
//use rocket::Redirect;

use std::path::PathBuf;
use std::path::Path;
use std::string::String;

mod db_manager;
use db_manager::create_user;
use db_manager::establish_connection;
use db_manager::get_user_by_email;
use db_manager::models::User;


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
INDEX CONTEXTS
*/
#[derive(Serialize)]
struct IndexContext {
    title: String,
    authenticated: bool
}

/*
INDEX GET AND POST
*/
#[get("/")]
fn index_get(mut cookies: Cookies) -> Template {
    let context = IndexContext {
        title: String::from("Index"),
        authenticated: logged_in(cookies),
    };
    Template::render("index", &context)
}


/*
REGISTER CONTEXTS & FORMS
*/
#[derive(Serialize)]
struct RegisterContext {
    title: String,
    authenticated: bool
}

#[derive(FromForm)]
struct RegisterForm {
    email: String,
    password: String,
    confirm_password: String,
}

fn logged_in(mut cookies: Cookies) -> bool {
    let cooks = cookies.get_private("user_id").is_none();
    if cookies.get_private("user_id").is_none() {
        return false
    }
    else {
        return true;
    }
    
}

/*
REGISTER GET AND POST
*/
#[get("/register")]
fn register_get(mut cookies: Cookies) -> Template {    
    let context = RegisterContext {
        title: String::from("Register"),
        authenticated: logged_in(cookies),
    };
    Template::render("register", &context)
}

#[post("/register", data = "<registerform>")]
fn register_post(registerform: Form<RegisterForm>, mut cookies: Cookies) -> String {

    // Get the form in a Rust useable format
    let register_form = &registerform.get();

    // Get the user email from register_form
    let email_input = register_form.email.to_string();

    // Get the password from the register form
    let password_input = register_form.password.to_string();

    // Get the password confirmation from the regsiter form
    let password_confirm = register_form.confirm_password.to_string();

    // Create new mutable empty string in message to display to user
    let mut message = String::new();
    
    // Check if the passwords match each other
    if &password_input == &password_confirm {
        // Establish connection to the DB
        let conn = establish_connection();

        let current_user = get_user_by_email(&email_input);
        println!("here");
        if current_user.email != "" {
            message = format!("{} already exists. Please login.", current_user.email);
        }
        else {
            //Add user to DB
            create_user(&conn, &email_input.to_string(), &password_input.to_string());
        
            //TODO: get user_id from the newly created user, and store it in a private_cookie DONE
            let current_user = get_user_by_email(&email_input);
            println!("Current User:\nid: {}\nemail: {}", current_user.id, current_user.email);
            cookies.add_private(Cookie::new("user_id", current_user.id.to_string()));

            //Create message indicating success
            message = format!("{} added as a new user", &email_input);
        }

        
    }
    else {
        //Create message indicating the passwords don't match
        message = format!("PASSWORDS DON'T MATCH!");
    }
    //TODO: Return a Redirect
    //TODO: Implement Flash messaging

    //Return the message, to be printed to the user
    return message
}
fn get_id_from_string(string_id: String) -> String {
    let temp = string_id.split_at(8).1;
    let mut id = String::new();
    for letter in temp.chars() {
        if letter.to_string() == ";" {
            break;
        }
        else {
            id.push(letter);
        }
    }
    return id

}


/*
LOGIN CONTEXT & FORMS
*/
#[derive(Serialize)]
struct LoginContext {
    title: String,
    authenticated: bool
}

#[derive(FromForm)]
struct LoginForm {
    email: String,
    password: String,
}

/*
LOGIN GET AND POST
*/
#[get("/login")]
fn login_get(mut cookies: Cookies) -> Template {
    let context = LoginContext {
        title: String::from("Login"),
        authenticated: logged_in(cookies),
    };
    Template::render("login", &context)
}

#[post("/login", data = "<loginform>")]
fn login_post(loginform: Form<LoginForm>, mut cookies: Cookies) -> String {
    //TODO: Implement Flash messaging
    
    
    let login_form = &loginform.get();
    let email_input = login_form.email.to_string();
    let password_input = login_form.password.to_string();
    let user = get_user_by_email(&email_input);
    let mut message = String::new();
    //TODO: Compare user, and password to DB records DONE
    if user.email == "" {
        //user doesn't exist in DB
        message = String::from("Email, or Password incorrect");
    }
    else {
        if user.password == password_input {
            //user exists, and password matches
            message = format!("{} logged in", user.email);
            cookies.remove_private(Cookie::named("user_id"));
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
        }
        else {
            //user exists, but password doesn't match
            message = String::from("Email, or Password incorrect");
        }
    }
    //TODO: Return a Redirect not a string
    message
}

/*
LOGOUT GET
*/
#[get("/logout")]
fn logout_get(mut cookies: Cookies) -> String {
    cookies.remove_private(Cookie::named("user_id"));
    String::from("USER ID COOKIE REMOVED")
}

/*
FOUNDATION CSS, and JS REQUESTS
*/
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        index_get,
        register_get,
        register_post,
        login_get,
        login_post,
        files,
        logout_get,
    ])
    .attach(Template::fairing())
}


fn main() {
    rocket().launch();
}
