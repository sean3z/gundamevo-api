use rocket::fs::NamedFile;

#[post("/Option/GetOptions", format = "json")]
pub async fn getoptions() -> Option<NamedFile> {
    NamedFile::open(r"C:\Users\Sean\Desktop\gundamevo\api\src\options\getoptions.response.json").await.ok()
}