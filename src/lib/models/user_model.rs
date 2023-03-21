use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseModel {
    pub user_name: String,
    pub teams: Vec<String>,
    pub last_name: String,
    pub email: String,
    pub first_name: String,
    pub statistics: HashMap<String, i64>,
    pub riot_puuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub id: String,
    pub match_date: String,
    pub damage: i64,
    pub earnings: i64,
    pub kills: i64,
    pub match_id: String,
    pub match_result: bool,
}

pub struct StatisticsWithAggregateModel {
    pub total_damage: i64,
    pub total_earnings: i64,
    pub total_kills: i64,
    pub total_wins: i64,
    pub total_losses: i64,
}
