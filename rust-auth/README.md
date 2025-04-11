# Rust Auth

A simple authentication API built with Rust using Rocket, SQLx, and JWT for token-based authentication.

## Features

- User registration and login with password hashing (Argon2)
- JWT-based authentication
- PostgreSQL database integration
- Health check endpoint

## Prerequisites

- Rust and Cargo installed
- PostgreSQL database setup
- `.env` file with the following variables:

```env
DATABASE_URL=postgres://user:password@localhost/database_name
JWT_SECRET=""
DB_USER=""
DB_PASSWORD=""
DB_HOST=""
DB_NAME="rust_auth"
```

## Dependencies

````bash
[dependencies]
argon2 = "0.5.3"
chrono = { version = "0.4.40", features = ["serde"] }
dotenv = "0.15.0"
jsonwebtoken = "9.3.1"
rocket = { version = "0.5.1", features = ["json"] }
serde = { version = "1.0.218", features = ["derive"] }
serde_json = "1.0.140"
sqlx = { version = "0.8.3", features = ["postgres", "runtime-tokio-native-tls","chrono"] }
'''


## Installation

```bash
# Clone the repository
git clone https://github.com/mrathod05/RUST-PROJECTS.git --branch dev
cd beginner/rust-auth


# Database
sqlx database create --database-url postgres://postgres:postgres@localhost/rust-auth
sqlx migrate run


# Build the project
cargo build --release

# Run the executable
./target/release/rust-auth
````

## API Endpoints

- Root Endpoint
  - GET /
  - Returns a welcome message
- Health Check

  - GET /health-check
  - Checks if the API is running

- Sign Up
  - POST /api/auth/sign-up
  - Request Body (JSON):
  ```json
  {
    "username": "Test",
    "email": "rust1@gmail.com",
    "password": "Test@123"
  }
  ```
  - Response:
  ```json
  {
    "code": 200,
    "success": true,
    "message": "Signup Successful",
    "data": null
  }
  ```
- Sign In
  - POST /api/auth/sign-in
  - Request Body (JSON):
  ```json
  {
    "email": "rust1@gmail.com",
    "password": "Test@123"
  }
  ```
  - Response:
  ```json
  {
    "code": 200,
    "success": true,
    "message": "SignIn Successful",
    "data": {
      "id": 1,
      "username": "Test",
      "email": "rust1@gmail.com",
      "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOnsiaWQiOjEsInVzZXJuYW1lIjoiVGVzdCIsImVtYWlsIjoicnVzdDFAZ21haWwuY29tIn0sImV4cCI6MTc0MTMzMDIzNn0._tBu9aueZLYjvLn1fwwdXUCsEeWRtnPPx9XiwCVTjjo"
    }
  }
  ```

## Technologies Used

    - Rust (Rocket, SQLx, Serde, JSON Web Token, Argon2, Chrono)
    -  PostgreSQL
