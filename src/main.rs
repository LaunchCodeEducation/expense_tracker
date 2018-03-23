#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;



#[get("/")]
fn index_get() -> String {
    String::from("Index")
}


/*
REGISTER GET AND POST
*/
#[get("/register")]
fn register_get() -> String {
    String::from("Register")
}

#[post("/register_post")]
fn register_post() -> String {
    String::from("Register Post")
}

/*
LOGIN GET AND POST
*/
#[get("/login")]
fn login_get() -> String {
    String::from("Login")
}

#[post("/post")]
fn login_post() -> String {
    String::from("Login Post")
}






fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        index_get,
        register_get,
        login_get,
    ])
}


fn main() {
    rocket().launch();
}
