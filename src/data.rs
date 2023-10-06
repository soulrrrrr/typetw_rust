
use serde_json::{from_str, Value};

use include_dir::{include_dir, Dir};

use rand::Rng;
static LANG_DIR: Dir = include_dir!("resources");

pub fn get_data() -> (String, String, String) {
    let file = LANG_DIR.get_file("poems_5.json").unwrap().contents_utf8().unwrap();
    let data: Value = from_str(file).unwrap();
    let i = rand::thread_rng().gen_range(0..data["count"].to_string().parse::<i32>().unwrap()) as usize; 
    return (
        data["poems"][i]["keyboard"].to_string().trim_matches('"').to_string(),
        data["poems"][i]["zhuyin"].to_string().trim_matches('"').to_string(),
        data["poems"][i]["content"].to_string().trim_matches('"').to_string(),
    )
}
