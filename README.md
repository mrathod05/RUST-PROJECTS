# 🦀 RUST-PROJECT

This repository is a collection of Rust projects and mini-experiments that I'm building while learning the Rust programming language. Each folder contains a self-contained project that focuses on different aspects of Rust and systems programming.

## 📁 Project Structure

```

RUST-PROJECT/

├── todo-app

├── rust-auth

├── http-status-checker

├── P2P/

│ ├── 1_basic_p2p_chat

│ └── 2_bidirectional_p2p_chat

```

## 🚀 Projects

### 📡 http-status-checker

A simple CLI tool to check the HTTP status of provided URLs.

**Dependencies Used:**

- `reqwest` – for making HTTP requests

- `tokio` & `tokio-macros` – async runtime for concurrency

- `clap` – for CLI argument parsing

- `tui`, `crossterm` – for building a terminal UI

- `url` – for URL parsing and validation

**📚 Learning Highlights:**

- Building async command-line tools with Rust

- Handling user input with a terminal UI

- Parsing and validating URLs

- Error handling and futures in async Rust

---

### 🗨️ P2P/1_basic_p2p_chat

A basic peer-to-peer chat system using TCP sockets.

**Dependencies Used:**

- `tokio` – for async TCP socket handling

**📚 Learning Highlights:**

- Working with `tokio` TCP streams

- Basic networking and message passing

- Writing minimal peer-to-peer programs

- Handling async I/O and simple concurrency

---

### 🔁 P2P/2_bidirectional_p2p_chat

An extension of the basic P2P chat to allow full-duplex communication with encryption.

**Dependencies Used:**

- `tokio` – for asynchronous bidirectional sockets

- `clap` – for parsing CLI flags

- `aes-gcm`, `base64`, `rand` – for encryption and key generation

- `thiserror` – for custom error handling

**📚 Learning Highlights:**

- Secure message transmission using encryption

- Error management with custom error types

- Splitting a binary with `[[bin]]`

- Designing a more complex peer model

---

### 🔐 rust-auth

A lightweight authentication system built with Rust, featuring JWT authentication and password hashing.

**Dependencies Used:**

- `argon2` – for password hashing

- `chrono` – for date and time management

- `dotenv` – for environment variable management

- `jsonwebtoken` – for JWT token handling

- `rocket` – for web server and handling JSON

- `serde`, `serde_json` – for serializing/deserializing data

- `sqlx` – for database interaction (PostgreSQL)

**📚 Learning Highlights:**

- Building a full authentication system (login, signup) in Rust

- Managing database connections with `sqlx` and async queries

- Hashing passwords securely with `argon2`

- Creating and verifying JWT tokens for authentication

- Web frameworks in Rust with `rocket` and handling JSON data

---

### 📝 todo-app

A simple command-line TODO app that stores tasks in a file.

**Dependencies Used:**

- `colored` – for colored CLI output

**📚 Learning Highlights:**

- Basic file handling and task management

- Working with the `colored` crate for styled terminal output

- Building a simple, interactive CLI application

---

## 🧠 What I'm Learning

- Rust syntax and best practices

- Ownership, borrowing, and lifetimes

- Concurrency and async programming

- Crate ecosystem (`tokio`, `serde`, `clap`, `reqwest`, `rocket`, etc.)

- Error handling and pattern matching

- Networking and I/O

- Building secure systems with encryption and JWT

- Web and CLI application structures

## 📌 Note

This is a personal learning repo—projects may be in progress or not fully production-ready.

## 📚 Resources I Use

- [The Rust Book](https://doc.rust-lang.org/book/)

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

Feel free to check out the individual projects and follow along my Rust journey! 🦀💪
