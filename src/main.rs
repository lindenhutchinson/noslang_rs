use reqwest;
use scraper::{Html, Selector};
use tokio;
use std::env;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    let noswear = args.len() > 2;
    let client = reqwest::Client::new();
    // Fetch the HTML content of a webpage
    let url = "https://www.noslang.com";
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::CONTENT_TYPE, 
        reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded")
    );
    let noswear_val = if noswear { "noswear" } else { "" };
    
    let request_builder = client.post(url)
        .headers(headers)
        .body(format!("p={}&noswear={}",text, noswear_val));

    let response = request_builder.send().await?;
    // Ensure the request was successful
    if response.status().is_success() {
        // Read the response body as bytes
        let body = response.bytes().await?;

        // Convert the response body to a string
        let body_str = String::from_utf8_lossy(&body);
        // Parse the HTML content
        let document = Html::parse_document(&body_str);
        // Define a CSS selector to extract specific elements
        let selector = Selector::parse(".translation-text").unwrap();

        // Iterate over elements that match the CSS selector
        for element in document.select(&selector) {
            // Print the text content of each matching element
            println!("{}", element.text().collect::<String>());
        }
    } else {
        println!("Failed to fetch webpage: {}", response.status());
    }

    Ok(())
}
