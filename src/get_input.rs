use std::env;
use std::{error::Error, fs};
use std::path::Path;

fn req(day: u8) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day.to_string());
    let cookie = include_str!("../cookie");
    let request = reqwest::blocking::Client::new()
        .get(url)
        .header("Cookie", cookie)
        .send()
        .unwrap();
    Ok(request.text().unwrap())
}

pub fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
    let mut dir_path = "data".to_string();
    if env::var("sample").unwrap_or("false".to_string()) == "true" {
        dir_path = "data_sample".to_string();
    }
    let file_path_name = format!("{}/input{}.txt", dir_path, day.to_string());
    // let file_path_name = String::new();
    if Path::new(&file_path_name).is_file() {
        let body = fs::read_to_string(file_path_name).unwrap();
        Ok(body)
    }
    else {
        let body = req(day).unwrap();
        let _ = fs::write(file_path_name, &body).unwrap();
        Ok(body)
    }
}
