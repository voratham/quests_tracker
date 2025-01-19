# The repo is quests_tracker workshop project from rust-mastery-saga-backend course

## How to install packages

```sh
cargo build
cargo install diesel_cli --no-default-features --features postgres
```

## Setup disel

```sh
diesel setup
# after remove migrations directory on root because this project we will move it on /infrastructure/postgres/migrations

# changed path on diesel.toml

diesel migrate run
```

### How to migration db with diesel

```sh
cd ./src/infrastructure/postgres
diesel migration run # up
diesel migration redo # down
```

## How to see coverage

```sh
cargo install cargo-tarpaulin
cargo tarpaulin --out xml
```

## How to update packages

```sh
cargo update
```

## Unit test

```sh
cargo install cargo-tarpaulin
cargo tarpaulin --out xml # How to export coverage xml format
cargo tarpaulin --out html # How to export coverage html format
open tarpaulin-report.html # How to see html coverage ✅
```

## Build Dockerfile

```sh
docker build --progress=plain -t quests_tracker:v1.0.0 -f ./Dockerfile .
docker run -p 8080:8080 --env-file ./.env.local quests_tracker:v1.0.0
```

reference_course: https://www.udemy.com/course/rust-mastery-saga-backend-rust
