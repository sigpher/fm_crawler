use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {}

async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:131233@localhost/postgres")
        .await?;
    
    Ok(pool)
}
