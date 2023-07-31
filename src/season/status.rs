use rocket::serde::json::{Json};
use rocket::serde::{Serialize};
use chrono::prelude::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    currentSeasonNo: usize,
    seasonPassStatus: SeasonStatus
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SeasonStatus { 
    isPremium: bool,
    passId: usize,
    totalExp: usize
}

#[post("/SeasonPass/GetPassStatus", format = "json")]
pub fn status() -> Json<Ressa> {
    Json(Ressa {
        currentSeasonNo: 5,
        seasonPassStatus: SeasonStatus {
            isPremium: false,
            passId: 5,
            totalExp: 200
        }
    })
}