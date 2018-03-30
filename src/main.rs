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
use rocket::response::{NamedFile, Flash, Redirect};
use rocket::request::{Form, FlashMessage};
use rocket::http::{Cookie, Cookies};
//use rocket::Redirect;

use std::path::PathBuf;
use std::path::Path;
use std::string::String;

mod db_manager;
use db_manager::create_user;
use db_manager::establish_connection;
use db_manager::{get_user_by_email, get_user_by_id};
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

fn logged_in(mut cookies: Cookies) -> bool {
    let cooks = cookies.get_private("user_id").is_none();
    if cookies.get_private("user_id").is_none() {
        return false
    }
    else {
        return true;
    }
    
}

fn get_user_id_from_cookie(mut cookies: Cookies) -> String {
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
    let mut flash_message = String::new();
    // Unwrap result or else return a string that looks like: "no class&no flash message"
    flash_message = flash.map(|msg| format!("{}&{}", msg.name().to_string(), msg.msg().to_string()))
        .unwrap_or_else(|| "no class&No flash message".to_string());
    // Split the flash message into a flash message like: "User logged in" and a flash class like: "success"
    let string_split_position = flash_message.find('&');
    let flash_message_split = flash_message.split_at(string_split_position.unwrap_or_else(|| 0));

    let message = (flash_message_split.0.to_string(), flash_message_split.1.to_string());
    return message;
}

#[get("/register")]
fn register_get(flash: Option<FlashMessage>, mut cookies: Cookies) -> Template {
    //DONE: Make flash_message_breakdown function
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    // Depending on the flash class convert the flash_class into CSS readable code that can be passed to our Tera template
    let mut flash_class = String::new();
    if flash_type == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }
    
    // Create context that is passed to Tera Template
    let context = RegisterContext {
        title: String::from("Register"),
        authenticated: logged_in(cookies),
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
    };

    // Render our register Tera Template, and pass it the context
    Template::render("register", &context)
}

#[post("/register", data = "<registerform>")]
fn register_post(registerform: Form<RegisterForm>, mut cookies: Cookies) -> Result<Flash<Redirect>, Flash<Redirect>> {

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
        if current_user.email != "" {
            println!("Redirect to /register msg=An account with email already exists...");
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
            create_user(&conn, &email_input.to_string(), &password_input.to_string());
        
            //DONE: get user_id from the newly created user, and store it in a private_cookie
            let current_user = get_user_by_email(&email_input);
            println!("Current User:\nid: {}\nemail: {}", current_user.id, current_user.email);
            cookies.add_private(Cookie::new("user_id", current_user.id.to_string()));

            //Create message indicating success
            println!("Redirect to /register msg=User logged in");
            //DONE: Return a Flash Redirect
            let msg = Flash::success(Redirect::to("/home"), format!("{} logged in", current_user.email));
            return Ok(msg)
            //message = format!("{} added as a new user", &email_input);
        }

        
    }
    else {
        //Create message indicating the passwords don't match
        println!("Redirect to /register msg=Passwords don't match");
        //DONE: Return a Flash Redirect
        return Err(Flash::error(Redirect::to("/register"), "Passwords don't match"));
        //message = format!("PASSWORDS DON'T MATCH!");
    }

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
#[get("/login")]
fn login_get(flash: Option<FlashMessage>, mut cookies: Cookies) -> Template {
    //DONE: Make flash_message_breakdown function
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    // Depending on the flash class convert the flash_class into CSS readable code that can be passed to our Tera template
    let mut flash_class = String::new();
    if flash_type == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }

    // Create context that is passed to Tera Template
    let context = LoginContext {
        title: String::from("Login"),
        authenticated: logged_in(cookies),
        flash_class: flash_class.to_string(),
        flash_msg: flash_message.to_string(),
    };

    // Render our login Tera Template, and pass it the context
    Template::render("login", &context)
}

#[post("/login", data = "<loginform>")]
fn login_post(loginform: Form<LoginForm>, mut cookies: Cookies) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_form = &loginform.get();
    let email_input = login_form.email.to_string();
    let password_input = login_form.password.to_string();
    let user = get_user_by_email(&email_input);
    let mut message = String::new();
    //DONE: Compare user, and password to DB records
    if user.email == "" {
        //user doesn't exist in DB
        message = String::from("Email, or Password incorrect");
        //DONE: Return a Flash Redirect
        return Err(Flash::error(Redirect::to("/login"), message));
    }
    else {
        if user.password == password_input {
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
LOGOUT GET
*/
#[get("/logout")]
fn logout_get(mut cookies: Cookies) -> Result<Flash<Redirect>, Flash<Redirect>> {
    cookies.remove_private(Cookie::named("user_id"));
    return Ok(Flash::success(Redirect::to("/login"), "Successfully logged out".to_string()));
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
#[get("/home")]
fn home_get(flash: Option<FlashMessage>, mut cookies: Cookies) -> Template {
    let message = flash_message_breakdown(flash);
    let flash_message = message.1.get(1..).unwrap_or_else(|| "no class");
    let flash_type = message.0;

    let mut auth = false;

    let str_user_id = get_user_id_from_cookie(cookies);
    if str_user_id != "-1".to_string() {
        auth = true;
    }
    let int_user_id: i32 = str_user_id.trim().parse().expect("Not a number");
    let current_user = get_user_by_id(&int_user_id);

    let context = HomeContext {
        title: "Home".to_string(),
        authenticated: auth,
        flash_class: flash_type.to_string(),
        flash_msg: flash_message.to_string(),
        user_email: current_user.email,
    };

    return Template::render("home", &context);
}


/*
FOUNDATION CSS, and JS REQUESTS
DEPRECATED BECAUSE WE ARE USING A CDN
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
        home_get,
    ])
    .attach(Template::fairing())
}


fn main() {
    rocket().launch();
}
