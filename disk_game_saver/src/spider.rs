extern crate reqwest;
use reqwest::header;
use std::fs::File;
use std::io::copy;

pub fn search_game_name(game_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut res = Vec::new();
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let resp = client
        .get(format!(
            "https://www.igdb.com/search_autocomplete_all?q={}",
            game_name
        ))
        .send()?
        .json::<serde_json::Value>()?;
    // print!("{:#?}", resp);
    if let Some(game_suggest) = resp.get("game_suggest").and_then(|v| v.as_array()) {
        for suggestion in game_suggest {
            if let Some(name) = suggestion.get("value").and_then(|v| v.as_str()) {
                // println!("{}", name);
                res.push(String::from(name));
            }
        }
    }

    Ok(res)
}

// 获得游戏图片链接
fn get_img_url(game_name: &str) {}

fn download_img(img_url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", "https://gvcover.top/".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get(img_url).headers(headers).send()?;
    let mut dest = File::create("image.jpg")?;
    copy(&mut res.bytes().unwrap().as_ref(), &mut dest)?;

    Ok(true)
}

// extern crate reqwest;
// use reqwest::header;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut headers = header::HeaderMap::new();
//     let client = reqwest::blocking::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .build()
//         .unwrap();
//     let res = client
//         .get("https://gvcover.top/?s=只狼")
//         // .headers(headers)
//         .send()?
//         .text()?;
//     println!("{}", res);

//     Ok(())
// }

#[test]
fn test_search_game_name() {
    let res = search_game_name("只狼").unwrap();
    println!("{:#?}", res);
}

#[test]
fn test_download_img() {
    let res = download_img("https://media.cloudfiare-img.buzz/wordpress/wp-content/uploads/2019/03/675768_font_ccn.jpg").unwrap();
    println!("{:#?}", res);
}
