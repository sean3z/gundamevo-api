use rocket::fs::NamedFile;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ressa { 
    playerIds: Vec<usize>
}

#[post("/OwnMobileSuitSetting/GetSettingsList", format = "json", data = "<playerIds>")]
pub async fn getsettingslist(playerIds: Json<Ressa>) -> Option<NamedFile> {
    NamedFile::open(r"C:\Users\Sean\Desktop\gundamevo\api\src\player\ownmobilesuitsetting.response.json").await.ok()
}