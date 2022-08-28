use actix_web::{web, get, Responder, HttpResponse};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct IThemeParkId {
    id: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaitTime {
    id: String,
    wait_time: Option<u32>,
    status: Option<String>,
    active: bool,
    last_update: Option<String>,
    name: String,
    fast_pass: bool,
    meta: WaitTimeMeta
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaitTimeMeta {
    #[serde(rename(deserialize = "type"))]
    ride_type: String,
    longitude: f64,
    latitude: f64,
    entity_id: String,
    single_rider: bool
}

pub async fn fetch_wait_times(id: String) -> Result<Vec<WaitTime>, reqwest::Error> {
    let url = format!("https://api.themeparks.wiki/preview/parks/{}/waittime", id);

    let body: Vec<WaitTime> = reqwest::get(url).await?.json().await?;
    Ok(body)
}

#[get("/{id}")]
pub async fn get_wait_times(data: web::Path<IThemeParkId>) -> impl Responder {

    let res = fetch_wait_times(data.id.clone()).await;
    HttpResponse::Ok().json(res.unwrap())
    

}

#[get("/attractions/{id}")]
pub async fn get_wait_time_for(data: web::Path<IThemeParkId>) -> impl Responder {
    HttpResponse::Ok().body(format!("wait times for attraction {}", data.id))
}