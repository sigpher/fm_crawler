use config::{Config, File};
// use serde::Deserialize;

fn main() {
    let config = get_config();
    let re = config.get_table("regex").unwrap();
    // for key in t.keys(){
    //   println!("{}",t[key]);
    // }
   let item_id_re =  re["item_id_re"].origin().unwrap();
    let item_id_re = regex::Regex::new(item_id_re).unwrap();
}

// pub fn get_key<'a, T: Deserialize<'a>>(config: &Config, key: &str) -> T {
//     config.get::<T>(key).unwrap()
// }

pub fn get_config() -> Config {
    Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("构建配置错误")
}


