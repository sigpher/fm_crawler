use std::sync::{Arc, Mutex};

use anyhow::{Ok, Result};
use fm_crawler::{get_html, get_links, get_pool, get_standard_info, insert_data};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://down.foodmate.net/special/standard/42.html";
    let html = get_html(url).await.unwrap();
    let links = get_links(&html);
    // println!("{:?}", links);
    // let mut infos = Vec::new();
    let mut futs = Vec::with_capacity(1000);
    let pool = Arc::new(Mutex::new(get_pool().await));

    for link in links {
        let pool = pool.lock().unwrap().clone();
        let h = tokio::spawn(async move {
            let html = get_html(&link).await;
            if let Some(html) = html {
                let info = get_standard_info(&html);
                // infos.push(info)
                // println!("{:?}", info);
                insert_data(&pool, info).await;
            }
        });
        futs.push(h);
    }
    for fut in futs {
        let _hanle = tokio::join!(fut);
    }
    Ok(())
}
