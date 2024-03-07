![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

## Playing Around with Turso DB

This is a simple project to play around with Turso DB. It's a simple key-value store that uses a file as a backend. It's a simple project to play around with Rust and databases.

## Running the Project

To run the project, you need to have Rust installed. Then, you can run the following command:

- Add a .env file with the following content:

you can get the `TURSO_DB_URL` and `TURSO_AUTH_TOKEN` from the Turso DB website or the CLI tool by running.

```bash
turso db show DB_Name
turso db tokens create DB_Name
```

```env
TURSO_DB_URL=...
TURSO_AUTH_TOKEN=...
```

- Run the project:

```bash
cargo run
```
