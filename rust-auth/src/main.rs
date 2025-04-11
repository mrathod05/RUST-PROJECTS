mod db;
mod response;
mod routes;
mod services;
mod utils;

use rocket::Config;
use routes::{auth, home};

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db::connection::establish_connection().await?;

    let config = Config::figment()
        .merge(("port", 8000))
        .merge(("address", "127.0.0.1"));

    let _ = rocket::custom(config)
        .manage(db_pool)
        .mount("/", routes![home::index, home::heath_check])
        .mount("/api/auth", routes![auth::sing_up, auth::sing_in])
        .launch()
        .await;

    Ok(())
}
