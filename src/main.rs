use anyhow::{Ok, Result};
use fm_crawler::{get_html, get_links, get_standard_info};

#[tokio::main]
// async fn main() -> Result<()> {
//     let url = "http://down.foodmate.net/special/standard/42.html";
//     let html = get_html(url).await.unwrap();
//     let links = get_links(&html);
//     // println!("{:?}", links);
//     // let mut infos = Vec::new();
//     for link in links {
//         let html = get_html(&link).await;
//         if let Some(html) = html {
//             let info = get_standard_info(&html);
//             // infos.push(info)
//             println!("{:?}", info);
//         }
//     }
//     Ok(())
// }

async fn main() -> Result<()> {
    let url = "http://down.foodmate.net/special/standard/42.html";
    let html = get_html(url).await.unwrap();
    let links = get_links(&html);
    // println!("{:?}", links);
    // let mut infos = Vec::new();
    let mut futs = Vec::with_capacity(1000);
    for link in links {
        let h = tokio::spawn(async move {
            let html = get_html(&link).await;
            if let Some(html) = html {
                let info = get_standard_info(&html);
                // infos.push(info)
                println!("{:?}", info);
            }
        });
        futs.push(h);
    }
    for fut in futs{
        let _hanle = tokio::join!(fut);
    }
    Ok(())
}
