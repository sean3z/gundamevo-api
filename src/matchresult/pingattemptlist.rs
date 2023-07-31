use rocket::fs::NamedFile;

#[post("/Matching/PingAttemptList", format = "json")]
pub async fn pingattemptlist() -> Option<NamedFile> {
    NamedFile::open(r"C:\Users\Sean\Desktop\gundamevo\api\src\matchresultpingattemptlist.response.json").await.ok()
}