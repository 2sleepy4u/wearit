use db::get_db_connection;
use edgedb_protocol::model::Json;
use rocket::serde::{Deserialize, json};
use rocket::http::CookieJar;
use edgedb_tokio::Queryable;

#[path = "./auth.rs"] mod auth;
use auth::isAuth;

pub const GET_USER_CLOTHES_QUERY: &str = "
with
  user := (
    select User 
    filter .<user[is Session].token = <str>$0
    limit 1
)
select Clothes {*}
filter .<clothes[is User] = user
";

pub const GET_USER_CLOTHES_GROUP_QUERY: &str = "
    with
      user := (
        select User 
        filter .<user[is Session].token = <str>$0
        limit 1
    )
    select (
      group (
        select Clothes {
          name,
          warmness,
          waterproof,
          color1,
          color2,
          color3
        }
        filter .<clothes[is User] = user
      ) by .bodyPart
    ) {
      item_group := .key.bodyPart,
      items := .elements
    }
";

#[get("/get_user_clothes", format = "json")]
pub async fn get_user_clothes(cookies: &CookieJar<'_>, auth: isAuth<'_>) -> String {
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


    if let Ok(result) = conn.query_json(GET_USER_CLOTHES_GROUP_QUERY, &(ssid,)).await {
        result.to_string()
    } else {
        "".to_string()
    }
}

const UPDATE_USER_CLOTHES: &str = "
    with
      user := (
        select User 
        filter .<user[is Session].token = <str>$0
        limit 1
      )
    update User
    filter .id = user.id
    set {
      clothes += (
        insert Clothes {
          name := <str>$1,
          warmness := <float32>$2,
          waterproof := <bool>$3,
          bodyPart := <str>$4,
          color1 := <HexColor>$5,
          color2 := <HexColor>$6,
          color3 := <HexColor>$7
        } unless conflict on .name else (
          update Clothes 
          set {
          warmness := <float32>$2,
          waterproof := <bool>$3,
          bodyPart := <str>$4,
          color1 := <HexColor>$5,
          color2 := <HexColor>$6,
          color3 := <HexColor>$7
          } 
        )
      )
    }
";

#[derive(Queryable)]
enum BodyPart {
    Hat,
    Shirt,
    Sweater,
    Jacket,
    Trousers,
    Socks,
    Shoes
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Clothes {
    name: String,
    warmness: f32,
    waterproof: bool,
    bodyPart: String,
    color1: String,
    color2: Option<String>,
    color3: Option<String>
}

#[post("/update_user_clothes", format = "json", data = "<payload>")]
pub async fn update_user_clothes(payload: json::Json<Clothes>, cookies: &CookieJar<'_>, auth: isAuth<'_>) {
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


    let color2 = match &payload.color2 {
        Some(color) => color,
        None => ""
    };

    let color3 = match &payload.color3 {
        Some(color) => color,
        None => ""
    };

    let query_params= &(
        ssid,
        &payload.name,
        &payload.warmness,
        &payload.waterproof,
        &payload.bodyPart,
        &payload.color1,
        &color2,
        &color3
        );

    let result = conn.execute::<_>(UPDATE_USER_CLOTHES, query_params).await;
    match result {
        Ok(expr) => println!("Success"),
        Err(e) => panic!("Errore: {}", e),
    }
}
