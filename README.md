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



reference_course: https://www.udemy.com/course/rust-mastery-saga-backend-rust

