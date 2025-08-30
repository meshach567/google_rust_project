use axum::{ routing::{get, post}, Router};

mod vehicle;

// use vehicle::{vehicle_get, vehicle_post};

#[tokio::main]

pub async fn main() {
    // Create the axum router
    let router01 = Router::new().route("/vehicle", get(vehicle::vehicle_get))
    .route("/vehicle", post(vehicle::vehicle_post));

    //2. Define the IP and port to listen on
    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // 3. axum serve to launch the web server
    axum::serve(listener, router01).await.unwrap();
}

