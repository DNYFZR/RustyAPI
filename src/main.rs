// JSON API interactions

mod api;
use serde_json;

fn main() {
    // API params
    let base_url = "https://dummyjson.com/products/";
    let header_key = "Content-Type"; 
    let header_value = "application/json";
    let product_endpoint = "1";

    let url = format!("{}{}", base_url, product_endpoint);

    // Get reqest
    let resp = api::get_json(&url);

    println!("Get Request");
    println!("{:?}", resp);

    // Post request
    let post_url = "https://dummyjson.com/products/add";
    let body:serde_json::Result<serde_json::Value> = serde_json::from_str("{\"title\": \"Iphone Galaxy + 1\"}");
    let resp = api::post_json(post_url, header_key, header_value, body.expect("JSON").to_string());

    println!("Post Request");
    println!("{:?}", resp);

    // Put request
    let body:serde_json::Result<serde_json::Value>  = serde_json::from_str("{\"title\": \"iPhone Galaxy +1\"}");
    let resp = api::put_json(&url, header_key, header_value, body.expect("JSON").to_string());

    println!("Put Request");
    println!("{:?}", resp);


    // Delete request
    let body:serde_json::Result<serde_json::Value>  = serde_json::from_str("{\"title\": \"iPhone Galaxy +1\"}");
    let resp = api::delete_json(&url, header_key, header_value, body.expect("JSON").to_string());

    println!("Delete Request");
    println!("{:?}", resp);
}


