use rocket::serde::json::{Json};
use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    battleRating: usize,
    battleRatingInfo: BattleRating,
    battleRatingMax: usize,
    battleRatingPrevMax: usize,
    eventUnionId: isize,
    exp: usize,
    gainEventPoint: usize,
    openType: usize,
    penaltyRemainingTime: String,
    playerIconItemId: String,
    playerId: usize,
    playerLevel: usize,
    playerName: String,
    portraitItemId: String,
    pretendOffline: bool,
    trophyItemId: String,
    useMobileSuitInfos: Vec<String>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BattleRating { 
    allSeasonsHighestBattleRating: usize,
    allSeasonsHighestRanking: usize,
    allSeasonsHighestSeason: usize,
    allSeasonsHighestTierType: usize,
    battleRating: usize,
    currentSeasonHighestBattleRating: usize,
    currentSeasonHighestRanking: usize,
    currentSeasonHighestTierType: usize,
    placementMatchCount: usize,
    ranking: usize,
    tierType: usize
}


#[post("/PlayerInformation/Self", format = "json")]
pub fn playerinformation() -> Json<Ressa> {
    Json(Ressa {
        battleRating: 0,
        battleRatingInfo: BattleRating {
            allSeasonsHighestBattleRating: 0,
            allSeasonsHighestRanking: 0,
            allSeasonsHighestSeason: 0,
            allSeasonsHighestTierType: 0,
            battleRating: 1700,
            currentSeasonHighestBattleRating: 1700,
            currentSeasonHighestRanking: 0,
            currentSeasonHighestTierType: 8,
            placementMatchCount: 0,
            ranking: 0,
            tierType: 8
        },
        battleRatingMax: 0,
        battleRatingPrevMax: 0,
        eventUnionId: -1,
        exp: 4700,
        gainEventPoint: 10,
        openType: 2,
        penaltyRemainingTime: "00:00:00".to_string(),
        playerIconItemId: "IT08_0290_v01".to_string(),
        playerId: 1523373,
        playerLevel: 10,
        playerName: "sean3z".to_string(),
        portraitItemId: "IT10_PT0081_v00".to_string(),
        pretendOffline: false,
        trophyItemId: "IT12_TR3003_11_v00".to_string(),
        useMobileSuitInfos: vec![]
    })
}