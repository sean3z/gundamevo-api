use rocket::serde::json::{Value, json};

#[post("/Logout", format = "json")]
pub fn logout() -> Value {
    json!({})
}