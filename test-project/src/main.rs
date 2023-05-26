use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Horoscope {
    sign: String,
    message: String,
    lucky_number: Option<u32>,
    lucky_animal: Option<String>,
}

async fn get_horoscope(sign: &str, plan: &str) -> Result<Horoscope, Box<dyn Error>> {
    let client: Client = Client::new();
    let url: &str = "https://matheusamendola-super-engine-q96gv49g7xvcg77-5000.preview.app.github.dev/horoscope";

    let body: serde_json::Value = json!({
        "sign": sign,
        "plan": plan,
    });

    let response: reqwest::Response = client.post(url).json(&body).send().await?;
    let horoscope: Horoscope = response.json().await?;

    Ok(horoscope)
}

#[tokio::main]
async fn main() {
    let sign: &str = "Áries";
    let plan: &str = "advanced";

    match get_horoscope(sign, plan).await {
        Ok(horoscope) => {
            println!("Horoscope: {:?}", horoscope);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
