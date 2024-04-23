use log::info;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};

#[tokio::main]
async fn main() -> RedisResult<()> {
    env_logger::init();
    // let client = redis::Client::open("redis://:131233@13672808880.imwork.net:53323")?;
    // let mut conn = client.get_connection()?;
    // set_data().await?;
    // println!("hello");

    Ok(())
}

pub async fn set_data() -> RedisResult<()> {
    let mut conn = get_conn().await;
    info!("Get connected");
    conn.lpush("kids", &["troy", "allen"]).await?;
    Ok(())
}

pub async fn get_conn() -> MultiplexedConnection {
    let client = redis::Client::open("redis://:131233@13672808880.imwork.net:53323").unwrap();
    // let conn = client.get_multiplexed_tokio_connection().await;
    let conn = client.get_multiplexed_async_connection().await.unwrap();
    conn
}

pub async fn get_data() -> RedisResult<()> {
    let mut conn = get_conn().await;
    info!("Get connected");
    let name: String = conn.get("name").await?;
    println!("{name}");

    Ok(())
}

// async fn get_all_keys() -> RedisResult<()> {
//     let mut conn = get_conn().await;
//     let keys: Vec<Standard> = conn.keys("foodmate:*").await?;
//     Ok(())
// }

