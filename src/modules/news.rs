use crate::cli::NewsCmd;
use crate::error::{AppError, AppResult};
use crate::utils::output::{info, warn};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Feed {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    url: String,
}

pub fn handle(cmd: NewsCmd) -> AppResult<()> {
    match cmd {
        NewsCmd::Latest => latest(),
    }
}

pub fn latest() -> AppResult<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| AppError::Network(e.to_string()))?;

    let resp = client
        .get("https://archlinux.org/feeds/news/")
        .send()
        .map_err(|e| AppError::Network(e.to_string()))?;

    if !resp.status().is_success() {
        return Err(AppError::Network(format!(
            "HTTP {}",
            resp.status()
        )));
    }

    let text = resp.text().map_err(|e| AppError::Network(e.to_string()))?;

    let feed: Feed = match serde_xml_rs::from_str(&text) {
        Ok(f) => f,
        Err(_) => {
            warn("Failed to parse Arch news feed.");
            return Ok(());
        }
    };

    info("Latest Arch News:");
    for item in feed.items.iter().take(10) {
        println!("- {}", item.title);
        println!("  {}", item.url);
    }

    Ok(())
}
