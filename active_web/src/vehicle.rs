use axum::Json;
use axum::debug_handler;

#[derive(Debug, serde::Serialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: String,
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    println!("Caller retrived a vehicle from axum");
    Json::from(
    Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RAM 1500".to_string(),
        year: 2024,
        id: uuid::Uuid::new_v4().to_string(),
    })
}

pub async fn vehicle_post() {

}