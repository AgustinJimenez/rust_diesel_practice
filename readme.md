### install dependencies

- cargo build

### install cargo-watch if necessary (for the next command)

- cargo install cargo-watch

### install diesel cli if isn't already installed

- brew install diesel

### generate .env file

- cp .env.example .env

### setup database (check the .env.example file for db credentials)

- diesel setup
- diesel migration run

### run server

- cargo watch -x run

### framework docs

- https://rocket.rs/v0.5-rc/guide/configuration/

### diesel orm docs

- https://diesel.rs/guides/getting-started

### run tests

- cargo test -- --nocapture

### docker

- docker build -t myapp .
