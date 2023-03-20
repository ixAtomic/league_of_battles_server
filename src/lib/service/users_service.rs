use std::borrow::Borrow;

use crate::dal::users;
use crate::home::home::{HomePageResponse, StatisticsAggregateResponse};
use crate::models::user_model::{StatisticsModel, UserResponseModel};
use chrono::{prelude::*, Duration};
use serde_dynamo::{self, from_item};

pub async fn get_user_data(user_id: &str) -> Result<HomePageResponse, Box<dyn std::error::Error>> {
    let user = get_user(user_id).await;
    let matches = users::get_matches(&user.riot_puuid, "0", "20")
        .await
        .unwrap();
    let user_stats = construct_statistics_model(&user.riot_puuid).await;
    let aggregate_stats = aggregate_stats(&user_stats, None, None);

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

fn subtract_days(date: NaiveDate, days: i64) -> NaiveDate {
    let duration = Duration::days(days);
    date - duration
}

fn get_first_index<I>(dates: I, today: NaiveDate, index: Option<usize>) -> usize
where
    I: IntoIterator,
    I::Item: Borrow<String>,
{
    if let Some(start_index) = index {
        let start_date = subtract_days(
            today,
            start_index
                .try_into()
                .expect("start_index should be able to parse to i64"),
        );
        dates
            .into_iter()
            .position(|rec| date_equals(rec.borrow(), start_date))
            .expect("result of start should be a usize value")
    } else {
        0
    }
}

fn date_equals(record_date: &str, start_date: NaiveDate) -> bool {
    if let Ok(date) = NaiveDate::parse_from_str(record_date, "%Y-%m-%d") {
        date == start_date
    } else {
        false
    }
}

fn aggregate_stats(
    stats: &StatisticsModel,
    start: Option<usize>,
    end: Option<usize>,
) -> StatisticsAggregateResponse {
    //TODO - Add something so that if a statistic is null and newly added it fills the statistic with empty values equal to the number of values currently in the table
    let today = Local::now().date_naive();
    let start_index = get_first_index(&stats.date, today, start);
    let end_index = stats.date.len() - get_first_index(stats.date.iter().rev(), today, end); //this will get the last index of the array based on the filter
    let (wins, losses) = stats.match_result[start_index..end_index].iter().fold(
        (0, 0),
        |(mut wins, mut losses), &result| {
            if result {
                wins += 1;
            } else {
                losses += 1;
            }
            (wins, losses)
        },
    );
    StatisticsAggregateResponse {
        total_damage: (stats.damage[start_index..end_index]) //start and ending indexes of the array which correspond to beginning and end points
            .iter()
            .fold(0, |tot, dmg| tot + dmg),
        total_earnings: stats.earnings[start_index..end_index]
            .iter()
            .fold(0, |tot, earn| tot + earn),
        total_kills: stats.kills[start_index..end_index]
            .iter()
            .fold(0, |tot, kills| tot + kills),
        total_wins: wins,
        total_losses: losses,
    }
}

async fn construct_statistics_model(puuid: &str) -> StatisticsModel {
    let result = users::get_user_statistics(puuid).await.unwrap_or_default();
    let result: StatisticsModel =
        from_item(result).expect("the result should parse to the Statistics Model");
    println!("Statistics Results: {:?}", result);
    return result;
}

pub async fn get_user(username: &str) -> UserResponseModel {
    let result = users::get_user(username).await.unwrap();
    let result: UserResponseModel =
        from_item(result).expect("the result should parse to the Response Model");
    return result;
}
