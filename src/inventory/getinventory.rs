use rocket::fs::NamedFile;

#[post("/Item/GetInventory", format = "json")]
pub async fn getinventory() -> Option<NamedFile> {
    NamedFile::open(r"C:\Users\Sean\Desktop\gundamevo\api\src\inventory\getinventory.response.json").await.ok()
}