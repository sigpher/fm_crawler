// use std::sync::{Arc, Mutex};
// use fm_crawler::{get_html, get_links, get_pool, get_standard_info, insert_data};

use anyhow::{Ok, Result};
use fm_crawler::{
    get_all_keys, get_conn, get_html, get_links, get_standard_info, set_data, show_data,
};
// use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init();
    // let url = "http://down.foodmate.net/special/standard/42.html";
    // let html = get_html(url).await.unwrap();
    // let links = get_links(&html);
    // // let mut infos = Vec::new();
    // let mut futs = Vec::new();
    // for link in links {
    //     info!("Scraping Page: {link}");
    //     let h = tokio::spawn(async move {
    //         let html = get_html(&link).await;
    //         if let Some(html) = html {
    //             let info = get_standard_info(&html);
    //             // println!("{:?}", info);
    //             set_data(info).await.unwrap();
    //         }
    //     });
    //     futs.push(h);
    // }
    // for fut in futs {
    //     let _hanle = tokio::join!(fut);
    // }

    let mut conn = get_conn().await;

    let keys = get_all_keys().await?;

    for key in keys {
        show_data(&mut conn, &key).await;
    }
    Ok(())
}

// async fn main() -> Result<()> {
//     env_logger::init();
//     let url = "http://down.foodmate.net/special/standard/42.html";
//     let html = get_html(url).await.unwrap();
//     let links = get_links(&html);
//     // let mut infos = Vec::new();

//     let mut futs = Vec::with_capacity(1000);
//     let pool = Arc::new(Mutex::new(get_pool().await));

//     for link in links {
//         info!("Scraping Page: {link}");
//         let pool = pool.lock().unwrap().clone();

//         let h = tokio::spawn(async move {
//             let html = get_html(&link).await;
//             if let Some(html) = html {
//                 let info = get_standard_info(&html);
//                 insert_data(pool, info).await;
//             }
//         });
//         futs.push(h);
//     }
//     for fut in futs {
//         let _hanle = tokio::join!(fut);
//     }
//     Ok(())
// }
