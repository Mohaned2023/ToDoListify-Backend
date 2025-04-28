use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub async fn get_pool() -> Pool<Postgres>{
    let db_url = std::env::var("TODOLISTIFY_DATABASE_URL")
        .expect(">>> TODOLISTIFY_DATABASE_URL NOT found!");
    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect(">>> Can NOT connect to database!")
}