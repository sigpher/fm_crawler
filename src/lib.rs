use log::info;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
use regex::Regex;
// use sqlx::{query, sqlite::SqlitePoolOptions, SqlitePool};
use std::{path::Path, time::Duration};

// const DATABASE: &'static str = "data.db";

// async fn get_html(url: &str) -> String {
pub async fn get_html(url: &str) -> Option<String> {
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.0.0";
    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .connect_timeout(Duration::from_secs(6))
        .referer(true)
        .build()
        .unwrap();

    let resp = client.get(url).send().await.unwrap();
    if resp.status().is_success() {
        let text = resp.text_with_charset("gb2312").await.unwrap();
        return Some(text);
    }
    None
}

pub fn get_links(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?s)class="bz_listl".*?<A.*?href="(?<link>.*?)""#).unwrap();
    re.captures_iter(html)
        .map(|c| c.name("link").unwrap().as_str().to_string())
        .collect()
}

pub fn get_standard_info(html: &str) -> Standard {
    let item_id_re = Regex::new(r"<script.*?item_id=(?<item_id>\d{3,}),").unwrap();
    let title_re = Regex::new(r#"(?s)title2.*?<span>(?<title>.*?)<font"#).unwrap();
    // let state_re = Regex::new(r#"(?s)<td bgcolor.*?<img src="(?<state_image>.*?)""#).unwrap();
    let status_re = Regex::new(r#"(?s)标准状态.*?<img src="(?<status_image>.*?)""#).unwrap();
    let published_at_re =
        Regex::new(r#"(?s)发布日期.*?(?<published_at>\d{4}-\d{2}-\d{2})"#).unwrap();
    let effective_at_re =
        Regex::new(r#"(?s)实施日期.*?(?<effective_at>\d{4}-\d{2}-\d{2})"#).unwrap();
    let issued_by_re =
        Regex::new(r##"(?s)颁发部门.*?<td bgcolor="#FFFFFF">(?<issued_by>.*?)</td>"##).unwrap();
    // let link_re = Regex::new(r#"(?s)class="downk.*?href="(?<link>.*?)""#).unwrap();

    let item_id = item_id_re
        .captures(html)
        .unwrap()
        .name("item_id")
        .unwrap()
        .as_str()
        .to_string();

    let title = title_re
        .captures(html)
        .unwrap()
        .name("title")
        .unwrap()
        .as_str()
        .to_string();

    let status = status_re
        .captures(html)
        .unwrap()
        .name("status_image")
        .unwrap()
        .as_str();

    let filename = Path::new(status).file_stem().unwrap().to_str().unwrap();

    let status = match filename {
        "bfyx" => "部分有效".to_string(),
        "jjfz" => "即将废止".to_string(),
        "jjss" => "即将生效".to_string(),
        "xxyx" => "现行有效".to_string(),
        "yjfz" => "已经废止".to_string(),
        _ => "".to_string(),
    };

    let published_at = published_at_re
        .captures(html)
        .unwrap()
        .name("published_at")
        .unwrap()
        .as_str()
        .to_string();

    let effective_at = effective_at_re
        .captures(html)
        .unwrap()
        .name("effective_at")
        .unwrap()
        .as_str()
        .to_string();

    let issued_by = issued_by_re
        .captures(html)
        .unwrap()
        .name("issued_by")
        .unwrap()
        .as_str()
        .to_string();

    // let link = link_re
    //     .captures(html)
    //     .unwrap()
    //     .name("link")
    //     .unwrap()
    //     .as_str()
    //     .to_string();

    Standard {
        item_id,
        title,
        status,
        published_at,
        effective_at,
        issued_by,
        // link,
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Standard {
    pub item_id: String,
    pub title: String,
    pub status: String,
    pub published_at: String,
    pub effective_at: String,
    pub issued_by: String,
    // pub link: String,
}

// pub async fn get_pool() -> sqlx::SqlitePool {
//     SqlitePoolOptions::new()
//         .max_connections(100)
//         .connect(DATABASE)
//         .await
//         .unwrap()
// }

// pub async fn insert_data(pool: SqlitePool, standard: Standard) {
//     query("INSERT INTO foodmate (title, status, published_at, effective_at,issued_by, link) VALUES($1,$2,$3,$4,$5,$6)")
//     .bind(standard.title)
//     .bind(standard.status)
//     .bind(standard.published_at)
//     .bind(standard.effective_at)
//     .bind(standard.issued_by)
//     .bind(standard.link)
//     .execute(&pool)
//     .await.unwrap();
// }

// pub async fn get_conn() -> RedisResult<MultiplexedConnection> {
//     let client = redis::Client::open("redis://:131233@13672808880.imwork.net:53323")?;
//     let conn = client.get_multiplexed_tokio_connection().await;
//     conn
// }

// pub async fn set_data(standard: Standard) -> RedisResult<()> {
//     let mut conn = get_conn().await?;
//     conn.hset_multiple(
//         format!("foodmate:{}", standard.item_id),
//         &[
//             ("title", standard.title),
//             ("status", standard.status),
//             ("published_at", standard.published_at),
//             ("effective_at", standard.effective_at),
//             ("issued_by", standard.issued_by),
//         ],
//     )
//     .await?;
//     info!("Get connected");
//     Ok(())
// }

pub async fn get_conn() -> MultiplexedConnection {
    // let client = redis::Client::open("redis://:131233@13672808880.imwork.net:53323").unwrap();
    let client = redis::Client::open("redis://:131233@127.0.0.1").unwrap();
    // let conn = client.get_multiplexed_tokio_connection().await;
    let conn = client
        .get_multiplexed_async_connection_with_timeouts(
            Duration::from_secs(5),
            Duration::from_secs(1),
        )
        .await
        .unwrap();
    conn
}

pub async fn set_data(standard: Standard) -> RedisResult<()> {
    let mut conn = get_conn().await;
    // conn.hset_multiple(
    conn.hset_multiple(
        format!("foodmate:{}", standard.item_id),
        &[
            ("title", standard.title),
            ("status", standard.status),
            ("published_at", standard.published_at),
            ("effective_at", standard.effective_at),
            ("issued_by", standard.issued_by),
        ],
    )
    .await?;
    info!("Get connected");
    Ok(())
}

pub async fn get_all_keys() -> RedisResult<Vec<String>> {
    let mut conn = get_conn().await;
    let keys: Vec<_> = conn.keys("foodmate:*").await?;

    Ok(keys)
}

pub fn count(keys: Vec<String>) -> u32 {
    let mut count = 0;
    for _ in keys {
        count += 1;
    }
    count
}

pub async fn show_data(conn: &mut MultiplexedConnection, key: &str) {
    let ( title, status, published_at, effective_at,issued_by): (
        String,
        String,
        String,
        String,
        String,
    ) = redis::pipe()
        .atomic()
        // .hget(key, "item_id")
        .hget(key, "title")
        .hget(key, "status")
        .hget(key, "published_at")
        .hget(key, "effective_at")
        .hget(key, "issued_by")
        .query_async(conn)
        .await
        .unwrap();
    println!("{title} {status} {published_at} {effective_at} {issued_by}");
}

// pub async fn insert_data(standard: Standard) {
//     let mut conn = get_conn().await;
//     let _ret:() = redis::pipe()
//         .atomic()
//         // .hget(key, "item_id")
//         .hset(format!("foodmate:{}",standard.item_id), "title",standard.title).ignore()
//         .hset(format!("foodmate:{}",standard.item_id), "status",standard.status).ignore()
//         .hset(format!("foodmate:{}",standard.item_id), "published_at",standard.published_at).ignore()
//         .hset(format!("foodmate:{}",standard.item_id), "effective_at",standard.effective_at).ignore()
//         .hset(format!("foodmate:{}",standard.item_id), "issued_by",standard.issued_by).ignore()
//         .query_async(&mut conn)
//         .await
//         .unwrap();
// }


// pub async fn set_data(standard: Standard) -> RedisResult<()> {
//     let mut conn = get_conn().await;
//     // conn.hset_multiple(
//     conn.hset_multiple(
//         format!("foodmate:{}", standard.item_id),
//         &[
//             ("title", standard.title),
//             ("status", standard.status),
//             ("published_at", standard.published_at),
//             ("effective_at", standard.effective_at),
//             ("issued_by", standard.issued_by),
//         ],
//     )
//     .await?;
//     info!("Get connected");
//     Ok(())
// }