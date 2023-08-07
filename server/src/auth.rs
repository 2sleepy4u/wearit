use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::{CookieJar, Cookie, Status};
use rocket::serde::{Deserialize, Serialize, json::Json};
use edgedb_tokio::Queryable;

use uuid::Uuid;
use db::get_db_connection;


pub const NEW_SESSION_QUERY: &str = "
 select exists (
  insert Session {
    token := <str>$0,
    user := assert_single((
      select User
      filter .email = <str>$1
    )),
    session_date := datetime_current()
  } unless conflict on .user else (
    update Session set {
      token := <str>$0,
      session_date := datetime_current()
    }
  )
) 
";

pub const CHECK_SESSION: &str = "
    select exists (
        select Session
        filter .token = <str>$0
        and .session_date > datetime_current() - <cal::relative_duration>'30 days'
        limit 1
      )
";


pub const CHECK_USER_QUERY: &str = "
    with
      session := (
        select Session
        filter .user.email = <str>$0 
        and .session_date > datetime_current() - <cal::relative_duration>'30 days'
        limit 1
      )
    select User {   
      email, 
      name,
      token := session.token if exists session else ''
    }
    filter .email = <str>$0 and .password = <str>$1
    limit 1
";


pub struct isAuth<'r>(pub &'r bool);

#[derive(Debug)]
pub enum AuthStatus {
    Unauthorized,
    Authorized
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for isAuth<'r> {
    type Error = AuthStatus;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        async fn is_valid(ssid: &str) -> bool {
            if ssid.is_empty() {
                return false;
            }

            let conn = get_db_connection().await;
            let conn = 
                match conn {
                    Ok(conn) => conn,
                    Err(_) => panic!("Unable to connect to db")
                };

            let result = conn.query_single::<bool, _>(CHECK_SESSION, &(ssid,)).await;
            let result = 
                match result {
                    Ok(Some(user)) => user,
                    Ok(None) => false,
                    Err(e) => panic!("Get user error: {}", e)
                };
            result
        }

        let ssid: &str = match req.cookies().get("SSID") {
            Some(cookie) => cookie.value(),
            None =>  ""
        };


        if is_valid(ssid).await {
            Outcome::Success(isAuth(&true))
        } else {
            Outcome::Failure((Status::Unauthorized, AuthStatus::Unauthorized))
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCredentials {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Token {
    token: String
}

#[derive(Clone, Debug, Queryable)]
struct User {
    email: String,
    name: String,
    token: String
}


#[post("/get_new_session", format = "json", data = "<payload>")]
pub async fn get_new_session(payload: Json<UserCredentials>, cookies: &CookieJar<'_>) -> Status {
    let conn = get_db_connection().await;

    let conn = 
        match conn {
            Ok(conn) => conn,
            Err(_) => panic!("Unable to connect to db")
        };
    
    let email = &payload.email;
    let password = &payload.password;

    let result = conn.query_single::<User, _>(CHECK_USER_QUERY, &(email, password,)).await;
    let user = 
        match result {
            Ok(Some(user)) => user,
            Ok(None) => panic!("No user found or wrong credential"),
            Err(e) => panic!("Get user error: {}", e)
        };

   
    let token = 
        if user.token.is_empty() {
            let token = Uuid::new_v4().to_string();
            let result = conn.query_single::<bool, _>(NEW_SESSION_QUERY, &(&token, &email)).await;

            if let Err(e) = result {
                panic!("Errore nella creazione della nuova sessione: {}", e);
            }
            token

        } else {
            user.token
        };

    let ssid = Cookie::build("SSID", token)
        .http_only(true)
        .finish();

    cookies.add(ssid);
    Status::Ok
}

