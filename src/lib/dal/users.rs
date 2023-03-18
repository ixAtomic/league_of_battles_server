use crate::config;
use aws_sdk_dynamodb::{model::AttributeValue, Client, Error};
use dotenv::dotenv;
use reqwest::{self};
use std::collections::HashMap;
use url::Url;

async fn connect() -> Result<Client, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

pub async fn users() -> Result<(), Error> {
    let client = connect().await?;
    let req = client.list_tables().limit(10);
    let resp = req.send().await?;
    println!("Current DynamoDB tables: {:?}", resp.table_names);
    Ok(())
}

pub async fn get_user(user_id: &str) -> Result<HashMap<String, AttributeValue>, Error> {
    let client = connect().await?;
    let result = client
        .get_item()
        .table_name("Users")
        .key("id", AttributeValue::S(user_id.to_owned()))
        .send()
        .await?;
    let result = result.item.unwrap();
    Ok(result)
}

pub async fn get_matches(
    puuid: &str,
    start: &str,
    count: &str,
) -> Result<Vec<String>, serde_json::Error> {
    // Load environment variables from .env file
    dotenv().ok();

    let base = Url::parse(&config::RIOT_BASE).unwrap();
    let matches_url = base
        .join(&format!(
            "/lol/match/v5/matches/by-puuid/{puuid}/ids?start={start}&count={count}"
        ))
        .unwrap();

    // Send a GET request to the specified URL and await the response
    let response = reqwest::Client::new()
        .get(matches_url)
        .header("X-Riot-Token", &config::RIOT_API_KEY.to_string())
        .send()
        .await
        .unwrap();

    // Return the response body as a string
    let body = response.text().await.unwrap();

    let matches: Vec<String> = serde_json::from_str(&body).unwrap();

    Ok(matches)
}
