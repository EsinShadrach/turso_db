use std::env;

use dotenv::dotenv;
use libsql::{params, Connection, Database};

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    dotenv().ok();
    match connect_to_db() {
        Ok(db) => {
            println!("Connected to database");

            let table_creation = db
                .execute(
                    "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
                    params![],
                )
                .await;

            match table_creation {
                Ok(x) => println!("Table created {}", x),
                Err(e) => println!("Error creating table: {}", e),
            }

            let user_insertion = db
                .execute("INSERT INTO users VALUES (?1, ?2)", params![1, "Rafe"])
                .await;

            match user_insertion {
                Ok(x) => println!("User inserted {}", x),
                Err(e) => println!("Error inserting user: {}", e),
            }
        }
        Err(e) => println!("Error connecting to database: {}", e),
    };

    Ok(())
}

fn connect_to_db() -> Result<Connection, libsql::Error> {
    let db_url = env::var("TURSO_DB_URL").expect("DATABASE_URL must be set");
    let db_token = env::var("TURSO_AUTH_TOKEN").expect("DATABASE_TOKEN must be set");
    let database = Database::open_remote(&db_url, &db_token);

    match database {
        Ok(db) => return db.connect(),
        Err(e) => return Err(e),
    }
}
