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
pub struct StatisticsModel {
    pub damage: Vec<i64>,
    pub date: Vec<String>,
    pub earnings: Vec<i64>,
    pub kills: Vec<i64>,
    pub match_id: Vec<String>,
    pub match_result: Vec<bool>,
}

pub struct StatisticsWithAggregateModel {
    pub total_damage: i64,
    pub total_earnings: i64,
    pub total_kills: i64,
    pub total_wins: i64,
    pub total_losses: i64,
}
