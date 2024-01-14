# 🦀 Axum-Postgres

A simple web server written in Rust using Axum and PostgreSQL.

The objective is to have a boilerplate for User CRUD and basic Auth.

## Usage

Setup an `.env` file and run:

```bash
# start database
docker-compose up -d

# start server
cargo run

# hot reloading
cargo-watch -c -q -w . -x "run"
```