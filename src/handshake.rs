use rocket::serde::json::{Json};
use rocket::serde::{Serialize};
use chrono::prelude::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    disabledMobileSuits: Vec<String>,
    enabledCustomMatchmake: bool,
    enabledMatchmake: bool,
    limitPackageVersionLogin: Vec<usize>,
    limitPackageVersionMatchmake: Vec<usize>,
    masterDataVersion: Vec<usize>,
    matchingAreaGuid: String,
    nextResponseSeconds: usize,
    opsNoticeCodes: Vec<String>,
    pushCode: String,
    secessionMatchPhase: usize,
    serverTimeUtcNow: DateTime<Utc>
}

#[post("/Handshake", format = "json")]
pub fn handshake() -> Json<Ressa> {
    Json(Ressa {
        disabledMobileSuits: vec![],
        enabledCustomMatchmake: true,
        enabledMatchmake: true,
        limitPackageVersionLogin: vec![
            1,
            0,
            0,
            244675
        ],
        limitPackageVersionMatchmake: vec![
            0,
            0,
            0,
            0
        ],
        masterDataVersion: vec![
            1,
            245947,
            1
        ],
        matchingAreaGuid: "9f5df1d8-1647-4e15-b63c-55f7a859fcc2".to_string(),
        nextResponseSeconds: 15,
        opsNoticeCodes: vec![
            "257_3".to_string(),
            "261_1".to_string(),
            "262_1".to_string(),
            "263_1".to_string()
        ],
        pushCode: "Poke".to_string(),
        secessionMatchPhase: 0,
        serverTimeUtcNow: Utc::now()
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Download { 
    downloadUrl: String,
    masterDataVersion: Vec<usize>
}

#[post("/MasterData/get", format = "json")]
pub fn masterdata() -> Json<Download> {
    Json(Download {
        downloadUrl: "https://packagedata.gundamevogame.com/HiaN4yRYXA3FP8zpdKewmiR-Bgh3f_ioRmkR18zLf4P".to_string(),
        masterDataVersion: vec![
            1,
            245947,
            1
        ]
    })
}