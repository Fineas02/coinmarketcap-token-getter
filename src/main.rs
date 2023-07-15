use reqwest;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Coin {
    id: u32,
    name: String,
    symbol: String,
    num_market_pairs: u32,
    quote: Quote,
}

#[derive(Debug, Deserialize)]
struct Quote {
    usd: Option<Usd>,
}

#[derive(Debug, Deserialize)]
struct Usd {
    price: f64,
    volume_24h: f64,
    market_cap: f64,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Coin>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = env::var("CMC_API_KEY").expect("CMC_API_KEY must be set"); // Add this line
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-CMC_PRO_API_KEY",
        HeaderValue::from_str(&api_key).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?start=1&limit=5000&sort=market_cap&market_cap_min=2000000&market_cap_max=20000000&volume_24h_min=500000"
    );

    let response = client.get(&url).send().await?;
    let api_response: ApiResponse = response.json().await?;

    let filtered_coins: Vec<Coin> = api_response
        .data
        .into_iter()
        .filter(|coin| coin.num_market_pairs >= 2)
        .collect();
    for coin in filtered_coins {
        println!("{:?}", coin);
    }

    Ok(())
}
