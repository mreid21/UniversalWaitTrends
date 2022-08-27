use actix_web::{web, get, Responder, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IThemeParkId {
    id: String,
}

#[get("/{id}")]
pub async fn get_wait_times(data: web::Path<IThemeParkId>) -> impl Responder {
    HttpResponse::Ok().body(format!("wait times for park {}", data.id))
}

#[get("/attractions/{id}")]
pub async fn get_wait_time_for(data: web::Path<IThemeParkId>) -> impl Responder {
    HttpResponse::Ok().body(format!("wait times for attraction {}", data.id))
}