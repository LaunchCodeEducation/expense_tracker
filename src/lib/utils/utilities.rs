use rocket::response::{Flash, Redirect};
use rocket::request::{FlashMessage};

pub fn not_logged_in(route: &str) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return Err(Flash::error(Redirect::to(route), "You need to login, or register!"));
}

pub fn improper_user_access(route: &str) -> Result<Flash<Redirect>, Flash<Redirect>> {
    return Err(Flash::error(Redirect::to(route), "You are already logged in! Logout to login, or create a new account."));
}

pub fn flash_message_breakdown(flash: Option<FlashMessage>) -> (String, String) {
    // Access the flash message result so it can be added to the context
    let flash_message: String;
    // Unwrap result or else return a string that looks like: "no class&no flash message"
    flash_message = flash.map(|msg| format!("{}&{}", msg.name().to_string(), msg.msg().to_string()))
        .unwrap_or_else(|| "no class&No flash message".to_string());
    // Split the flash message into a flash message like: "User logged in" and a flash class like: "success"
    let string_split_position = flash_message.find('&');
    let flash_message_split = flash_message.split_at(string_split_position.unwrap_or_else(|| 0));

    let flash_class: String;
    if flash_message_split.0.to_string() == "success".to_string() {
        flash_class = "success".to_string();
    }
    else {
        flash_class = "alert".to_string();
    }

    let message = (flash_class, flash_message_split.1.to_string());
    return message;
}