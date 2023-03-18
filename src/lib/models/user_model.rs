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
