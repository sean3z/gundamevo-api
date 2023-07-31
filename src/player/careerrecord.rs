use rocket::fs::NamedFile;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    storedSeasonNo: usize
}

#[post("/CareerRecord/GetSelf", format = "json", data = "<storedSeasonNo>")]
pub async fn getself(storedSeasonNo: Json<Ressa>) -> Option<NamedFile> {
    NamedFile::open(r"C:\Users\Sean\Desktop\gundamevo\api\src\player\careerrecord.response.json").await.ok()
}