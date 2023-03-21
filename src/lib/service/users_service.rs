use crate::dal::users;
use crate::home::home::{HomePageResponse, StatisticsAggregateResponse};
use crate::models::user_model::{Stats, UserResponseModel};
use serde_dynamo::aws_sdk_dynamodb_0_24::from_items;
use serde_dynamo::{self, from_item};

pub async fn get_user_data(user_id: &str) -> Result<HomePageResponse, Box<dyn std::error::Error>> {
    let user = get_user(user_id).await;
    let matches = users::get_matches(&user.riot_puuid, "0", "20")
        .await
        .unwrap();
    let user_stats = construct_statistics_model(&user.riot_puuid).await;
    let aggregate_stats = aggregate_stats(&user_stats);

    let reply = HomePageResponse {
        user_name: user.user_name,
        teams: user.teams,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        statistics: user.statistics,
        riot_puuid: user.riot_puuid,
        matches: matches,
        stats: Some(aggregate_stats),
    };

    Ok(reply)
}

fn aggregate_stats(stats: &Vec<Stats>) -> StatisticsAggregateResponse {
    let init = StatisticsAggregateResponse {
        total_damage: 0,
        total_earnings: 0,
        total_kills: 0,
        total_wins: 0,
        total_losses: 0,
    };

    let f = |acc: StatisticsAggregateResponse, stat: &Stats| StatisticsAggregateResponse {
        total_damage: acc.total_damage + stat.damage,
        total_earnings: acc.total_earnings + stat.earnings,
        total_kills: acc.total_kills + stat.kills,
        total_wins: acc.total_wins + if stat.match_result { 1 } else { 0 },
        total_losses: acc.total_losses + if stat.match_result { 0 } else { 1 },
    };

    stats.iter().fold(init, f)
}

async fn construct_statistics_model(puuid: &str) -> Vec<Stats> {
    let result = users::get_user_statistics(puuid, None, None)
        .await
        .unwrap_or_default();
    println!("result {:?}", result);
    let results: Vec<Stats> = from_items(result).expect("results should parse to Stats model");
    results
}

pub async fn get_user(username: &str) -> UserResponseModel {
    let result = users::get_user(username).await.unwrap();
    let result: UserResponseModel =
        from_item(result).expect("the result should parse to the Response Model");
    return result;
}
