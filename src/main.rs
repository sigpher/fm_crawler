// use std::sync::{Arc, Mutex};
// use fm_crawler::{get_html, get_links, get_pool, get_standard_info, insert_data};


use anyhow::{Ok, Result};
use fm_crawler::*;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init();
    // let url = "http://down.foodmate.net/special/standard/42.html";
    // let html = get_html(url).await.unwrap();
    // let links = get_links(&html);
    // // let mut infos = Vec::new();
    // let mut futs = Vec::new();
    // let  pool = get_sqlite_pool().await.unwrap();
    // for link in links {
    //     let pool = pool.clone();
    //     info!("Scraping Page: {link}");
    //     let h = tokio::spawn(async move {
    //         let html = get_html(&link).await;
    //         if let Some(html) = html {
    //             let info = get_standard_info(&html);
    //             // insert_data(&pool,info).await;
    //             insert_data_by_sqlite(&pool,info).await;
    //         }
    //     });
    //     futs.push(h);
    // }
    // for fut in futs {
    //     let _hanle = tokio::join!(fut);
    // }
    // pool.close().await;
    // show_data_by_pg().await;

    // let mut conn = get_conn().await;

    // let keys = get_all_keys().await?;

    // for key in keys {
    //     show_data(&mut conn, &key).await;
    
    // }
    show_data_by_sqlite().await;
    Ok(())
}
