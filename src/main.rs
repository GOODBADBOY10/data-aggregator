use axum::{Router, routing::get, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json;

// CoinGecko response structure
#[derive(Deserialize, Serialize, Debug)]
struct CoinGeckoData {
    bitcoin: CoinPrice,
    ethereum: CoinPrice,
}

#[derive(Deserialize, Serialize, Debug)]
struct CoinPrice {
    usd: f64,
    usd_market_cap: f64,
    usd_24h_vol: f64,
    usd_24h_change: f64,
}

// CryptoCompare response structure
#[derive(Deserialize, Serialize, Debug)]
struct CryptoCompareData {
    #[serde(rename = "USD")]
    usd: CryptoComparePrice,
}

#[derive(Deserialize, Serialize, Debug)]
struct CryptoComparePrice {
    #[serde(rename = "PRICE")]
    price: f64,
    #[serde(rename = "MKTCAP")]
    market_cap: f64,
    #[serde(rename = "SUPPLY")]
    supply: f64,
}
// dexcscreener response structure
#[derive(Deserialize, Serialize, Debug)]
struct Pair {
    chain_id: String,
    pair_address: String,
    base_token: Token,
    price_usd: String,
    volume: Volume,
    liquidity: Liquidity,
    fdv: i64,
    market_cap: i64,
    pair_created_at: i64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Token {
    address: String,
    name: String,
    symbol: String,
}


#[derive(Deserialize, Serialize, Debug)]
struct Volume {
    h24: f64,
    h6: f64,
    h1: f64,
    m5: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Liquidity {
    usd: f64,
    base: f64,
    quote: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct DesxcreenerData {
    schema_version: String,
    pairs: Vec<Pair>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AggregatedData {
    // From DexScreener
    dex_token_name: String,
    dex_token_symbol: String,
    dex_price_usd: String,
    dex_liquidity_usd: f64,
    dex_volume_24h: f64,
    dex_market_cap: i64,
    
    // From CoinGecko
    btc_price: f64,
    eth_price: f64,
    btc_24h_change: f64,
    
    // From CryptoCompare
    btc_supply: f64,
    btc_market_cap_cc: f64,
    
    // Metadata
    fetched_at: String,
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


async fn get_response() -> Result<impl IntoResponse, String> {
    println!("Fetching data from 3 APIs in parallel...");
    
    // Make 3 API calls in parallel using tokio::join!
    let (dex_result, coingecko_result, cryptocompare_result) = tokio::join!(
        // Call 1: DexScreener
        reqwest::get("https://api.dexscreener.com/latest/dex/tokens/0x84604526d71bbe7738c3c02d3c8a48778955718289c03d814d8468b58ae9a898::skelsui::SKELSUI"),
        
        // Call 2: CoinGecko (Bitcoin and Ethereum prices)
        reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true"),
        
        // Call 3: CryptoCompare (Bitcoin data)
        reqwest::get("https://min-api.cryptocompare.com/data/pricemultifull?fsyms=BTC&tsyms=USD")
    );
    
    // Parse DexScreener response
    let dex_data: DesxcreenerData = dex_result
    .map_err(|e| format!("Desxcreener result failed: {}", e))?
    .json()
    .await
    .map_err(|e| format!("Dexscreener request failed: {}", e))?;
    
    // Parse CoinGecko response
    let coingecko_data: CoinGeckoData = coingecko_result
        .map_err(|e| format!("CoinGecko request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("CoinGecko JSON parse failed: {}", e))?;
    
    // Parse CryptoCompare response
    let cryptocompare_response = cryptocompare_result
        .map_err(|e| format!("CryptoCompare request failed: {}", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("CryptoCompare JSON parse failed: {}", e))?;
    
    // Extract data from CryptoCompare (nested structure)
    let cc_data = &cryptocompare_response["RAW"]["BTC"]["USD"];
    
    // Get current timestamp
    let now = chrono::Utc::now().to_rfc3339();
    
    // Extract first pair from DexScreener (or handle if empty)
    let first_pair = dex_data.pairs.first()
        .ok_or("No pairs found in DexScreener response")?;
    
    // Create aggregated response
    let aggregated = AggregatedData {
        // DexScreener data
        dex_token_name: first_pair.base_token.name.clone(),
        dex_token_symbol: first_pair.base_token.symbol.clone(),
        dex_price_usd: first_pair.price_usd.clone(),
        dex_liquidity_usd: first_pair.liquidity.usd,
        dex_volume_24h: first_pair.volume.h24,
        dex_market_cap: first_pair.market_cap,
        
        // CoinGecko data
        btc_price: coingecko_data.bitcoin.usd,
        eth_price: coingecko_data.ethereum.usd,
        btc_24h_change: coingecko_data.bitcoin.usd_24h_change,
        
        // CryptoCompare data
        btc_supply: cc_data["SUPPLY"].as_f64().unwrap_or(0.0),
        btc_market_cap_cc: cc_data["MKTCAP"].as_f64().unwrap_or(0.0),
        
        // Metadata
        fetched_at: now,
    };
    
    println!("Successfully aggregated data from all 3 APIs!");
    
    // Return as JSON
    Ok(Json(aggregated))
}