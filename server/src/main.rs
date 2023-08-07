#[macro_use] extern crate rocket;

use std::thread;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::http::{Cookie, Header, Method};
use rocket::request::Request;

mod auth;
use auth::{isAuth, get_new_session};

mod user;
use user::{get_user_clothes, update_user_clothes};

mod weather_location;
use weather_location::get_info_weather;

mod GA;
use GA::routes::get_new_outfit;

#[get("/")]
fn index(auth: isAuth) -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong!"
}

#[catch(401)]
fn unauthorized(req: &Request) -> String {
    req.cookies().remove(Cookie::named("SSID"));
    "Ops Not Authorized".to_owned()
}

#[options("/<_..>")]
fn request_roll_preflight() {}

fn test() {
    let handle = thread::spawn(myPrint);

    println!("Hello from main");

    //handle.join().unwrap();
}

fn myPrint() {
    println!("Hello from thread");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_response("Add CORS", |req, response| Box::pin(async move {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        })))
        .mount("/", routes![index, ping, request_roll_preflight])
        .mount("/auth", routes![get_new_session])
        .mount("/api", routes![
               get_user_clothes, update_user_clothes, 
               get_info_weather, get_new_outfit
        ])
        .mount("/public", FileServer::from("public/"))
        .register("/", catchers![unauthorized])
}

