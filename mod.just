mod db
_default:
    just --list api

migrate:
    sqlx migrate run

build:
    cargo build --release

run:
    cargo run