use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Pair {
    chain_id: String,
    dex_id: String,
    url: String,
    pair_address: String,
    base_token: Token,
    quote_token: Token,
    price_native: String,
    price_usd: String,
    txns: Transactions,
    volume: Volume,
    price_change: PriceChange,
    liquidity: Liquidity,
    fdv: i64,
    market_cap: i64,
    pair_created_at: i64,
    info: Info,
}

#[derive(Deserialize, Serialize, Debug)]
struct Token {
    address: String,
    name: String,
    symbol: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Transactions {
    m5: TransactionCount,
    h1: TransactionCount,
    h6: TransactionCount,
    h24: TransactionCount,
}

#[derive(Deserialize, Serialize, Debug)]
struct TransactionCount {
    buys: i32,
    sells: i32,
}


#[derive(Deserialize, Serialize, Debug)]
struct Volume {
    h24: f64,
    h6: f64,
    h1: f64,
    m5: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct PriceChange {
    h1: Option<f64>,
    h6: Option<f64>,
    h24: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Liquidity {
    usd: f64,
    base: f64,
    quote: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Info {
    image_url: String,
    header: String,
    open_graph: String,
    websites: Vec<Website>
}

#[derive(Deserialize, Serialize, Debug)]
struct Website {
    url: String,
    label: String,
}


#[derive(Deserialize, Serialize, Debug)]
struct DesxcreenerData {
    schema_version: String,
    pairs: Vec<Pair>,
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

    let port = "127.0.0.1:3000";


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