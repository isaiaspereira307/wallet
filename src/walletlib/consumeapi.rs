use reqwest::Result;
use serde::Deserialize;
use tokio::runtime::Runtime;


#[derive(Deserialize, Debug)]
struct Ticket {
    success: bool,
    market: String,
    last: f64,
    high: f64,
    low: f64,
    vol: f64,
    avg: f64,
    var: f64,
    buy: f64,
    sell: f64,
    timestamp: String
}


impl Ticket {
    async fn request() -> Ticket {
        get_generic().await.unwrap()
    }
}


async fn get_generic() -> Result<Ticket> {
    let url = "https://api.bitpreco.com/btc-brl/ticker";
    let client = reqwest::Client::new();
    let response: Ticket = client
        .get(url)
        .header("content-type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}


pub fn pegar_valor_btc() -> f64 {
    let rt = Runtime::new().unwrap();
    let resultado = rt.block_on(Ticket::request());
    resultado.last
}