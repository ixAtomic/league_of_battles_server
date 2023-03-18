use crate::dal::users;
use crate::home::home::HomePageResponse;
use crate::models::user_model::UserResponseModel;
use serde_dynamo::{self, from_item};

pub async fn get_user_data(user_id: &str) -> Result<HomePageResponse, Box<dyn std::error::Error>> {
    let user = get_user(user_id).await;
    let matches = users::get_matches(&user.riot_puuid, "0", "20")
        .await
        .unwrap();

    let reply = HomePageResponse {
        user_name: user.user_name,
        teams: user.teams,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        statistics: user.statistics,
        riot_puuid: user.riot_puuid,
        matches: matches,
    };

    Ok(reply)
}

pub async fn get_user(username: &str) -> UserResponseModel {
    let result = users::get_user(username).await;
    let result: UserResponseModel =
        from_item(result.unwrap()).expect("the result should parse to the Response Model");
    return result;
}
