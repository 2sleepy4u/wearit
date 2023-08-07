use std::time::Instant;

use rocket::http::CookieJar;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[path = "./../libs.rs"] mod libs;
use db::get_db_connection;
use crate::weather_location::{get_city_coord, get_city_temperature};

use super::types::*;
use super::functions::*;
use super::{run_evolution, accuracy};

#[path = "./../auth.rs"] mod auth;
use auth::*;

const GET_USER_CLOTHES: &str = "
    with 
        user := (
            select User 
            filter .<user[is Session].token = <str>$0
            limit 1
        ),
        filteredClothes := ( 
          select Clothes filter .<clothes[is User] = user
        )
    select {
      hat := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Hat),
      shirt := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Shirt),
      sweater := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Sweater),
      jacket := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Jacket),
      trousers := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Trousers),
      socks := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Socks),
      shoes := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Shoes)
    } limit 1;
";

#[get("/get_new_outfit?<city>")]
pub async fn get_new_outfit(city: String, cookies: &CookieJar<'_>, auth: isAuth<'_>) -> Json<Vec<Gene>> {
    let conn = get_db_connection().await;
    let conn = 
        match conn {
            Ok(conn) => conn,
            Err(_) => panic!("Unable to connect to db")
        };

    let ssid = 
        if let Some(ssid) = cookies.get("SSID") {
            ssid.value()
        } else {
            panic!("Somethings wrong with user SSID");
        };


    let clothes_list = conn.query_single::<GeneList, _>(GET_USER_CLOTHES, &(ssid,)).await;

    let items: GeneList =
        if let Ok(Some(clothes)) = clothes_list {
            clothes
        } else {
            panic!("Error, no clothes found!");
        };

    let coord = get_city_coord(city).await;
    let res = match coord {
        Ok(pos) => get_city_temperature(pos).await,
        Err(e) => panic!("{}", e)
    };
    let forecast = match res {
        Ok(forecast) => forecast,
        Err(e) => panic!("{}", e)
    };

    let temp = forecast.hourly.temperature_2m[0].clone();
    let cold_level = (((temp - 40.0) / (-10.0 - 40.0)) * 4.0).abs() + 1.0;

    //let max_warmness = 20.0;

    let max_warmness = (cold_level * 7.0).round();
    let size = 10;
    let generation_limit = 100;
    let mutation_number = 1;
    let mutation_prob = 0.5;

    println!("Starting outfit generation request... T:{}-maxW:{}", temp, max_warmness);
    let time = Instant::now();
    let result = run_evolution(
        items,
        size,
        generation_limit,
        max_warmness,
        mutation_number,
        mutation_prob,
        fitness,
        generate_population,
        selection_pair,
        mutation,
        single_pair_crossover
        );

    println!("Time {:.2?}", time.elapsed());
    println!("Accuracy: {}", accuracy(result[0].clone(), max_warmness, fitness));
    println!("Ending outfit generation request.");
    Json(result[0].clone())
 
}
