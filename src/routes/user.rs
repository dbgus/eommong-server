use rocket_contrib::json::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct UserInfo {
    name: String,
    age: i32,
}
#[get("/")]
pub fn get_user() -> Json<UserInfo> {
    Json(UserInfo {
        name: "asda".to_string(),
        age: 18,
    })
}
