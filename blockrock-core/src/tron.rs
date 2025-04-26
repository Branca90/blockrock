use reqwest::Client;
use serde_json::Value;

/// Fetches the balance of a TRON address from the Nile Testnet using TronGrid API.
/// 
/// # Arguments
/// * `address` - The TRON address to query.
/// * `api_key` - The TronGrid API key (must be set via the TRONGRID_API_KEY environment variable).
/// 
/// # Returns
/// * `Result<String, reqwest::Error>` - The balance in sun (or an error message if the request fails).
pub async fn get_tron_balance(address: &str, api_key: &str) -> Result<String, reqwest::Error> {
    if api_key.is_empty() {
        return Ok("Error: API key is empty".to_string());
    }

    let client = Client::new();
    let url = format!("https://nile.trongrid.io/v1/accounts/{}", address);
    let response = client.get(&url)
        .header("TRON-PRO-API-KEY", api_key)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: Value = resp.json().await?;
                if json["data"].as_array().map_or(false, |data| data.is_empty()) {
                    Ok("0".to_string()) // Indirizzo esiste ma non ha fondi
                } else {
                    Ok(json["data"][0]["balance"].to_string())
                }
            } else {
                Ok(format!("HTTP Error: {}", resp.status()))
            }
        }
        Err(e) => Ok(format!("Request Error: {}", e)),
    }
}
