# Spellbook API

Spellbook is social media platform for wizards to not only keep track of their own spells but also share and copy them with and from others.

## Requirements

- git
- gcc
- libpq-dev (PostgreSQL library)
- [Rustup](https://rustup.rs/) (Rust version manager)
- [Docker](https://www.docker.com/) + [Docker Compose](https://docs.docker.com/compose/install/)
- [Diesel CLI](https://diesel.rs/guides/getting-started)

## Installation

1. Clone git repository

```
git clone https://github.com/schwoens/spellbook-api
cd spellbook-api
```

2. Prepare .env file

```
cp .env.d .env
```

3. (Optional) Change Postgres Username and Password in .env file
  
4. Build project

```
cargo build
```
5. Start docker container
```
docker-compose up -d
```

6. Setup database
```
diesel setup
```

## Troubleshooting
- Make sure you have everything mentioned above installed or the project will not compile.

## Usage

### Start the server
```
docker-compose up -d
cargo run
```

For testing it is recommened to use the Swagger UI 'Try it out' feature. You can find the Swagger documentation at [http://localhost:8080](http://localhost:8080) when the docker container is running.

