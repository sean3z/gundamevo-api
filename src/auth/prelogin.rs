use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task { 
    accountType: String,
    authToken: String,
    currentLanguage: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Terms { 
    countryCode: usize,
    images: Vec<String>,
    languageCode: usize,
    majorVersion: usize,
    minorVersion: usize,
    platform: usize,
    termId: usize,
    text: String,
    title: String,

    // TODO: rename this to 'type' (rust reserved keyword)
    type_name: usize 
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    agreedServiceTerm: bool,
    countryCode: String,
    matchingArea: usize,
    returnBattle: bool,
    season: usize,
    serviceTerms: Vec<Terms>
}

#[post("/PreLogin", format = "json", data = "<task>")]
pub fn prelogin(task: Json<Task>) -> Json<Ressa> {
    Json(Ressa {
        agreedServiceTerm: true,
        countryCode: "US".to_string(),
        matchingArea: 1,
        returnBattle: false,
        season: 5,
        serviceTerms: vec![Terms {
            countryCode: 0,
            images: vec![],
            languageCode: 0,
            majorVersion: 4,
            minorVersion: 3,
            platform: 1,
            termId: 3,
            text: "BANDAI NAMCO ONLINE INC.\r\nTERMS OF SERVICE\r\n\r\n\r\nLast updated: September 6, 2022\r\n\r\nPLEASE READ THESE TERMS OF SERVICE CAREFULLY.".to_string(),
            title: "TERMS OF SERVICE".to_string(),
            type_name: 0
        }]
    })
}