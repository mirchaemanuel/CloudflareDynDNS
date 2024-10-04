use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::time::Duration;
use std::error::Error;

#[derive(Deserialize)]
struct Config {
    public_ip_service: String,
    cloudflare_api_key: String,
    zone_id: String,
    dns_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config("config.toml")?;

    let public_ip = get_public_ip(&config.public_ip_service).await?;
    println!("Public IP: {}", public_ip);

    let record_id = get_record_id(&config).await?;

    update_cloudflare_record(&config, &record_id, &public_ip).await?;

    Ok(())
}

fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

async fn get_public_ip(service_url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let response = client.get(service_url).send().await?.text().await?;
    Ok(response.trim().to_string())
}

async fn get_record_id(config: &Config) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}", config.zone_id, config.dns_name);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.cloudflare_api_key))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await?;
        if let Some(record) = response_json["result"].as_array().and_then(|arr| arr.first()) {
            if let Some(record_id) = record["id"].as_str() {
                return Ok(record_id.to_string());
            }
        }
        Err("Record ID not found".into())
    } else {
        Err(format!("Failed to get DNS record ID: {}", response.text().await?).into())
    }
}

async fn update_cloudflare_record(config: &Config, record_id: &str, ip: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", config.zone_id, record_id);

    let body = serde_json::json!({
        "type": "A",
        "name": config.dns_name,
        "content": ip,
        "ttl": 120,
        "proxied": false
    });

    let response = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", config.cloudflare_api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        println!("DNS record updated successfully.");
    } else {
        eprintln!("Failed to update DNS record: {}", response.text().await?);
    }

    Ok(())
}