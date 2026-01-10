mod db
_default:
    just --list

migrate:
    sqlx migrate run

build:
    cargo build --release

run:
    cargo run