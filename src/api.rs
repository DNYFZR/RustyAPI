use reqwest::blocking::{get, Client, Response};
use reqwest::Error;
use serde_json;

pub fn get_json(url: &str) -> Result<serde_json::Value, reqwest::Error>{
    let res: serde_json::Value = get(url)?.json()?;
    
    Ok(res)
}

pub fn post_json(url: &str, header_key:&str, header_value:&str, body: String) -> Result<Response, Error>{
    let cli = Client::new();
    let res = cli
        .post(url)
        .header(header_key, header_value)
        .body(body)
        .send()?;
    
    Ok(res)
}


pub fn put_json(url: &str, header_key:&str, header_value:&str, body: String) -> Result<Response, Error>{
    let cli = Client::new();
    let res = cli
        .put(url)
        .header(header_key, header_value)
        .body(body)
        .send()?;
    
    Ok(res)
}

pub fn delete_json(url: &str, header_key:&str, header_value:&str, body: String) -> Result<Response, Error>{
    let cli = Client::new();
    let res = cli
        .delete(url)
        .header(header_key, header_value)
        .body(body)
        .send()?;
    
    Ok(res)
}

