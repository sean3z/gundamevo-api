use rocket::fs::NamedFile;

#[post("/MatchResult/GetSecessionPlayersMatchProgress", format = "json")]
pub async fn secession() -> Option<NamedFile> {
    NamedFile::open(r"C:\Users\Sean\Desktop\gundamevo\api\src\matchresult\secessionplayersmatch.json").await.ok()
}