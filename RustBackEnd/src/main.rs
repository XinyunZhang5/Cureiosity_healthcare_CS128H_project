use axum::{Router, routing::get, routing::post};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(routes::login))
        .route("/chat", post(routes::chat));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
