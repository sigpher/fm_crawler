use regex::Regex;
use std::time::Duration;


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
    let title_re = Regex::new(r#"(?s)title2.*?<span>(?<title>.*?)<font"#).unwrap();
    // let state_re = Regex::new(r#"(?s)<td bgcolor.*?<img src="(?<state_image>.*?)""#).unwrap();
    let state_re = Regex::new(r#"(?s)标准状态.*?<img src="(?<state_image>.*?)""#).unwrap();
    let published_at_re =
        Regex::new(r#"(?s)发布日期.*?(?<published_at>\d{4}-\d{2}-\d{2})"#).unwrap();
    let effective_at_re =
        Regex::new(r#"(?s)实施日期.*?(?<effective_at>\d{4}-\d{2}-\d{2})"#).unwrap();
    let issued_by_re =
        Regex::new(r##"(?s)颁发部门.*?<td bgcolor="#FFFFFF">(?<issued_by>.*?)</td>"##).unwrap();
    let link_re = Regex::new(r#"(?s)class="downk.*?href="(?<link>.*?)""#).unwrap();

    let title = title_re
        .captures(html)
        .unwrap()
        .name("title")
        .unwrap()
        .as_str()
        .to_string();

    let state = state_re
        .captures(html)
        .unwrap()
        .name("state_image")
        .unwrap()
        .as_str()
        .to_string();

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

    let link = link_re
        .captures(html)
        .unwrap()
        .name("link")
        .unwrap()
        .as_str()
        .to_string();

    Standard {
        title,
        state,
        published_at,
        effective_at,
        issued_by,
        link,
    }
}

#[derive(Debug)]
pub struct Standard {
    pub title: String,
    pub state: String,
    pub published_at: String,
    pub effective_at: String,
    pub issued_by: String,
    pub link: String,
}
