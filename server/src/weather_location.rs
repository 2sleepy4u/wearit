use url::Url;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct GeoPosition {
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct GeoResult {
    pub results: Vec<GeoPosition>
}
pub async fn get_city_coord(city: String) -> anyhow::Result<GeoPosition> {
    let url = "https://geocoding-api.open-meteo.com/v1/search";
    let params = &[
        ("name", city),
        ("count", 1.to_string())
    ];

    let url = Url::parse_with_params(url, params)?;
    let res = reqwest::get(url)
        .await?
        .json::<GeoResult>()
        .await?;
    let position = res.results[0].clone();
    Ok(position)
}
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HourlyForecast {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub precipitation_probability: Vec<i32>,
    pub windspeed_10m: Vec<f32>
}
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct WeatherForecast {
    pub hourly: HourlyForecast
}

// https://api.open-meteo.com/v1/forecast?latitude=45.6495&longitude=13.7768&hourly=temperature_2m,precipitation_probability,windspeed_10m&timezone=Europe%2FBerlin&forecast_days=1
pub async fn get_city_temperature(coord: GeoPosition) -> anyhow::Result<WeatherForecast>{
    let url = "https://api.open-meteo.com/v1/forecast";
    let params = &[
        ("latitude", coord.latitude.to_string()),
        ("longitude", coord.longitude.to_string()),
        ("hourly", "temperature_2m".to_string() + ",precipitation_probability" + ",windspeed_10m"),
    ];

    let url = Url::parse_with_params(url, params)?;
    let res = reqwest::get(url)
        .await?
        .json::<WeatherForecast>()
        .await?;
    Ok(res)
}

//TODO format = json
#[get("/get_info_weather?<city>")]
pub async fn get_info_weather(city: String) -> Json<WeatherForecast> {
    let coord = get_city_coord(city).await;
    let res = match coord {
        Ok(pos) => get_city_temperature(pos).await,
        Err(e) => panic!("{}", e)
    };
    let forecast = match res {
        Ok(forecast) => forecast,
        Err(e) => panic!("{}", e)
    };

    Json(forecast)
    
}

