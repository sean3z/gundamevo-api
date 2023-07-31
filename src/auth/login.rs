use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task { 
    CreateConsoleSession: bool,
    accountType: String,
    authToken: String,
    cpuInfo: String,
    gpuInfos: Vec<String>,
    hddUuid: String,
    macAddress: Vec<usize>,
    matchingArea: usize,
    memInfo: String,
    osInfo: String,
    packageVersion: Vec<usize>,
    platformInfo: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    apiServerVersion: String,
    bonusProgress: usize,
    disabledMobileSuits: Vec<String>,
    initialLevel: usize,
    isNewSeason: bool,
    matchingArea: usize,
    opsNoticeCodes: Vec<String>,
    penaltyRemainingTime: String,
    playerId: usize,
    privilegeLevel: usize,
    returnBattle: bool,
    secessionMatchPhase: usize,
    serverTimeUtcNow: DateTime<Utc>,
    token: String,
    tutorialProgress: usize,
    warning: bool,
    warningText: String,
    warningTitle: String
}

#[post("/Login", format = "json", data = "<task>")]
pub fn login(task: Json<Task>) -> Json<Ressa> {
    Json(Ressa {
        apiServerVersion: "230711-093520 (f2951dd1)".to_string(),
        bonusProgress: 2,
        disabledMobileSuits: vec![],
        initialLevel: 3,
        isNewSeason: false,
        matchingArea: 1,
        opsNoticeCodes: vec![
            "257_3".to_string(),
            "261_1".to_string(),
            "262_1".to_string(),
            "263_1".to_string()
        ],
        penaltyRemainingTime: "00:00:00".to_string(),
        playerId: 1523373,
        privilegeLevel: 0,
        returnBattle: false,
        secessionMatchPhase: 0,
        // serverTimeUtcNow: "2023-07-29T23:41:37.3230358Z".to_string(),
        serverTimeUtcNow: Utc::now(),
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzZXNzaW9uSWQiOiJOb3J0aEFtZXJpY2E6QzpVUzoyam5zWXlIWGwrQ2RCQ0QxTC9zbytQeDhpRjFuc0lYR3pOUFl4TjRISDZFPSIsIm1hc3RlckRhdGFIYXNoIjoiMCIsInVybCI6ImFwaS9BdXRoL0VPUyIsInNlcnZlciI6InByb2R1Y3Rpb24iLCJzdWIiOiJwcm9kdWN0aW9uXzE1MjMzNzMiLCJuaWNrbmFtZSI6InNlYW4zeiIsImV4cCI6MTY5MDY3NDE1NywiaXNzIjoiZXZvLWFwaS1zZXJ2ZXIiLCJhdWQiOiJldm8tcGxheWVyIn0.89nMd5E3TRnq70mw-eL6bSGL22_FRCwlYyxK6JxGdPo".to_string(),
        tutorialProgress: 15,
        warning: false,
        warningText: "".to_string(),
        warningTitle: "".to_string()
    })
}