use fm_crawler::Standard;
use log::info;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
    env_logger::init();
    let pool = get_pool().await.unwrap();
    println!("get pool");
    // let _rec = sqlx::query("INSERT INTO foodmate (item_id, title, status, published_at, effective_at, issued_by) VALUES (10000, 'GB 5009.00 食品安全XX标准 永不存在', '现行有效', '2023-01-01', '2024-01-01', '国家XXXXXXXXX');").execute(&pool).await.unwrap();
    // info!("Insert data successful");
    show_data_by_pg().await;
    pool.close().await;
}

async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:131233@localhost:5432/postgres")
        .await?;
    info!("connect database success");
    Ok(pool)
}

async fn show_data_by_pg() {
    let pool = get_pool().await.unwrap();
    println!("get pool");
    let info:Vec<_> = sqlx::query("SELECT * from foodmate")
        .fetch_all(&pool)
        .await
        .unwrap();
    info!("finished");
}

