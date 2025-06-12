use reqwest::Client;
use serde_json::Value;

pub async fn get_tron_balance(address: &str, apikey: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://nile.trongrid.io/v1/accounts/{}", address);
    let response = client
        .get(&url)
        .header("TRON-PRO-API-KEY", apikey)
        .send()
        .await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
            if !data.is_empty() {
                if let Some(balance) = data[0].get("balance") {
                    return Ok(balance.to_string());
                }
            }
        }
        Ok("0".to_string())
    } else {
        Ok(format!("HTTP Error: {}", response.status()))
    }
}

