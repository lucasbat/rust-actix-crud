use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let check_migrate = sqlx::migrate!("./src/databases/postgres_connection/migrations")
        .run(&pool)
        .await;

        match check_migrate {
            Ok(_) => {
                println!("Migrations run successfully");
            },
            Err(err) => {
                println!("Migrations failed with error: {:?}", err); // Added missing fat arrow
            }
        }
        pool
}