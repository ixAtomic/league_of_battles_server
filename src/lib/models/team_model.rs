use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct TeamResponseModel {
    pub username: String,
    pub teams: Vec<String>,
    pub last_name: String,
    pub email: String,
    pub first_name: String,
    pub statistics: HashMap<String, i64>,
}
