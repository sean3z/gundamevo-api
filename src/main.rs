#![allow(non_camel_case_types)]

#[macro_use] extern crate rocket;
extern crate chrono;

mod auth;
mod handshake;
mod notices;
mod player;
mod matchresult;
mod season;
mod options;
mod inventory;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            handshake::handshake,
            handshake::masterdata,
            notices::notices,
            player::information::playerinformation,
            player::ownmobilesuitsetting::getsettingslist,
            player::careerrecord::getself,
            matchresult::secession::secession,
            matchresult::pingattemptlist::pingattemptlist,
            season::status::status,
            options::getoptions::getoptions,
            inventory::getinventory::getinventory
        ])
        .mount("/api/Auth", routes![
            auth::prelogin::prelogin,
            auth::login::login,
            auth::logout::logout,
        ])
}