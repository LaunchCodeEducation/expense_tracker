#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::path::PathBuf;
use std::path::Path;
use std::string::String;
use rocket::request::Form;

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
    title: String
}

/*
INDEX GET AND POST
*/
#[get("/")]
fn index_get() -> Template {
    let context = IndexContext {
        title: String::from("Index"),
    };
    Template::render("index", &context)
}


/*
REGISTER CONTEXTS & FORMS
*/
#[derive(Serialize)]
struct RegisterContext {
    title: String
}

#[derive(FromForm)]
struct RegisterForm {
    email: String,
    password: String,
    confirm_password: String,
}

/*
REGISTER GET AND POST
*/
#[get("/register")]
fn register_get() -> Template {
    let context = RegisterContext {
        title: String::from("Register"),
    };
    Template::render("register", &context)
}

#[post("/register", data = "<registerform>")]
fn register_post(registerform: Form<RegisterForm>) -> String {
    //TODO: Implement Flash messaging
    //TODO: Write User information to the DB
    //TODO: Return a Redirect not a String
    let register_form = &registerform.get();
    let email_input = register_form.email.to_string();
    let password_input = register_form.password.to_string();
    let password_confirm = register_form.confirm_password.to_string();
    let message = format!("INPUT EMAIL: {}\nINPUT PASS: {}\nINPUT CONFIRM: {}", email_input, password_input, password_confirm);
    //println!("{}", &message);
    return message
}

/*
LOGIN CONTEXT & FORMS
*/
#[derive(Serialize)]
struct LoginContext {
    title: String
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
fn login_get() -> Template {
    let context = LoginContext {
        title: String::from("Login"),
    };
    Template::render("login", &context)
}

#[post("/login", data = "<loginform>")]
fn login_post(loginform: Form<LoginForm>) -> String {
    //TODO: Implement Flash messaging
    //TODO: Compare user, and pass to DB records
    //TODO: Return a Redirect not a string
    let login_form = &loginform.get();
    let email_input = login_form.email.to_string();
    let password_input = login_form.password.to_string();
    format!("EMAIL INPUT: {}\nPASS INPUT: {}", email_input, password_input)
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
    ])
    .attach(Template::fairing())
}


fn main() {
    rocket().launch();
}
