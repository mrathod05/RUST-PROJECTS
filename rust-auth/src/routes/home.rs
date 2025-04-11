use rocket::http::Status;

#[get("/")]
pub async fn index() -> String {
    String::from("Welcome to Rust Auth API!")
}

#[get("/heath-check")]
pub async fn heath_check() -> Status {
    Status::Ok
}
