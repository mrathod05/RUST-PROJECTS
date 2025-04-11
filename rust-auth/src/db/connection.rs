use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, process::Command};

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    // Get database configuration
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "my_app_db".to_string());

    // Create database URL
    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        db_user, db_password, db_host, db_name
    );
    let pg_url = format!(
        "postgres://{}:{}@{}/postgres",
        db_user, db_password, db_host
    );

    // First try to create the database
    println!("Creating database if it doesn't exist...");

    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&pg_url)
        .await
    {
        Ok(pool) => {
            // Check if database exists
            let query = format!("SELECT 1 FROM pg_database WHERE datname = '{}'", db_name);
            let exists: Option<(i32,)> = sqlx::query_as(&query).fetch_optional(&pool).await?;

            if exists.is_none() {
                let create_db = format!("CREATE DATABASE {}", db_name);
                sqlx::query(&create_db).execute(&pool).await?;
                println!("Database created successfully!");
            } else {
                println!("Database already exists!");
            }
        }
        Err(e) => {
            // Option 2: Fall back to command line if sqlx connection fails
            println!("Couldn't connect with sqlx: {}", e);
            println!("Trying with psql command...");

            let status = Command::new("psql")
                .arg("-U")
                .arg(&db_user)
                .arg("-h")
                .arg(&db_host)
                .arg("-c")
                .arg(format!("CREATE DATABASE {} IF NOT EXISTS", db_name))
                .status()?;

            if status.success() {
                println!("Database created via psql!");
            } else {
                println!("Warning: Could not create database. It may already exist.");
            }
        }
    };

    // Connect to the specific database
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Run migrations
    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("✅ Database setup complete!");

    Ok(pool)
}
