<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/rust-lang-ve/design/main/assets/logo_above.png" height="120" width="120" />
  </div>
  <h1 align="center">rust-mart-server</h1>
  <h4 align="center">Simple e-commerce server made with Actix</h4>
</div>

## Requirements
- Docker
- Rust

## Development

Build and run with `cargo`:

```bash
cargo run
```

### Setup Diesel

Using cargo install command, install `diesel_cli` binary:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

If theres no `diesel.toml` file in the project directory, run diesel setup to generate such file.

Then run migrations issuing:

```bash
diesel migration run
```

> In order to create a new migration issue: diesel `migration generate <migration name>`

## Endpoints

Method | URL | Req. Body | Res. Body
--- | --- | --- | ---
GET | `/auth/login` | - | `{"username": String, "password": String }`

## Environment Variables

Description of the environment variables defined in the `.env.sample`.

Key | Description
--- | ---
`POSTGRES_USER` | PostgreSQL Database User
`POSTGRES_PASSWORD` | PostgreSQL Database Password
`DATABASE_URL` | PostgreSQL Database URL (Connection String)

## System Requirements

Depending on your operative system you will probably need to install some packages
to run this project.

### Ubuntu 20

The following commands must be issued to have all the dependencies in place
to build this project with `cargo build` on an Ubuntu 20 system.

```bash
# install build-essential
sudo apt install build-essential
```

```bash
# instal openssl ref: https://docs.rs/crate/openssl/0.9.24
sudo apt-get install pkg-config libssl-dev
```

```bash
# install libpq-dev to use diesel and postgresql
sudo apt install libpq-dev
```
