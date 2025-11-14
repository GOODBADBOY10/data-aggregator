use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Dexscreener {
    name: String,
    mcap: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coingecko {
    price: u64,
    symbol: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Insidex {
    liquidity: f64,
    trending: Vec<u64>,
}



#[tokio::main]
async fn main() {
    println!("Entry of thr main file");

    let port = "0.0.0.0:3000";


    let app = Router::new()
    .route("/", get(get_root))
    .route("/response", get(get_response));

    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



async fn get_root() -> &'static str {
    println!("Hello from root");
    "Root endpoint hit"
}


async fn get_response() {}