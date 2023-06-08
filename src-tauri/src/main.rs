#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;
use serde::{Deserialize, Serialize};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Tauri!", name)
}

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    description: String,
    content: String,
    image: String,
    publishedAt: String,
    url: String,
    source: Source,
}

#[derive(Deserialize, Serialize)]
struct TopHeadlinesResponseOrWhatever {
    totalArticles: i32,
    articles: Vec<Article>
}

#[tauri::command]
async fn get_latest_news() -> Vec<Article> {
    let request_url = format!("https://gnews.io/api/v4/top-headlines?category=general&lang=en&max=30&apikey=");
    println!("{}", request_url);

    let response = reqwest::get(&request_url).await;

    let response = match response {
        Ok(response) => response,
        Err(e)=> panic!("Failed to get news because {} !", e)
    };
    
    let maybe_articles: Result<TopHeadlinesResponseOrWhatever, _> = response.json().await;

    let articles = match maybe_articles {
        Ok(data) => data.articles,
        Err(e) => panic!("Falied to parse json because {} !", e)
    };

    println!("{:?}", articles);
   
    return articles;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_latest_news])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
