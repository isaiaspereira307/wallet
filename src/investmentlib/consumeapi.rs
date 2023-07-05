use reqwest::Result;
use serde::Deserialize;
use tokio::runtime::Runtime;


#[derive(Deserialize, Debug)]
struct Selic {
        data:  String,
        valor: String,    
}

impl Selic {
    async fn request() -> Vec<Selic> {
        get_selic().await.unwrap()
    }
}


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
        get_btc().await.unwrap()
    }
}


async fn get_btc() -> Result<Ticket> {
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

async fn get_selic() -> Result<Vec<Selic>> {
    let url = "https://api.bcb.gov.br/dados/serie/bcdata.sgs.4189/dados/ultimos/10?formato=json";
    let client = reqwest::Client::new();
    let response: Vec<Selic> = client
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


pub fn pegar_valor_selic() -> f32 {
    let rt = Runtime::new().unwrap();
    let resultado = rt.block_on(Selic::request());
    resultado[9].valor.parse::<f32>().unwrap()
}


// pub async fn get_fund_mov<T: serde::de::DeserializeOwned>(option: &'static str, client_id: u16, month: u32, year: i32) -> Result<Vec<T>> {
//     let start_date = get_month_range(month, year).unwrap().0.format("%d/%m/%Y");
//     let end_date = get_month_range(month, year).unwrap().1.format("%d/%m/%Y");
//     let endpoint = match option {
//         "fundos" => format!(
//             "{}?client_id={}&start_date={}&end_date={}", FUNDOS, client_id, start_date, end_date
//         ),
//         "movimentacoes" => format!(
//             "{}?client_id={}&start_date={}&end_date={}", MOVIMENTACOES, client_id, start_date, end_date
//         ),
//         _ => "invalido".to_string(),  
//     };
//     let url = format!(
//         "{}/{}", BASE_URL, endpoint
//     );
//     let client = reqwest::Client::new();
//     let response: Vec<T> = client
//         .get(url)
//         .header(ACCESS, TOKEN)
//         .header(CONTENT_TYPE, "application/json")
//         .header(ACCEPT, "application/json")
//         .send()
//         .await?
//         .json()
//         .await?;
//     Ok(response)
// }