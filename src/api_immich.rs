use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub assets: AssetPage,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPage {
    pub items: Vec<Asset>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    pub created_at: String,
//     pub updated_at: String,
}

pub fn search_assets(
    client: &Client,
    server_url: &str,
    api_key: &str,
    person_id: &str,
    last_created_at: Option<&str>,
    cursor: Option<&str>,
) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/api/search/metadata",
        server_url.trim_end_matches('/')
    );

    let mut body = json!({
        "filter": {
            "personIds": { "any": [person_id] }
        }
        // , "size": 100,
    });

    if let Some(last_created_at) = last_created_at {
        body["filter"]["createdAt"] = json!({
            "gt": last_created_at
        });
    }

    if let Some(cursor) = cursor {
        body["cursor"] = json!(cursor);
    }

    /*
        client
        .post(url)
        .header("x-api-key", api_key)
        .json(&body)
        .send()?
        .error_for_status()?
        .json::<SearchResponse>()
    */

    let response = client
        .post(url)
        .header("x-api-key", api_key)
        .json(&body)
        .send()?;

    let status = response.status();
    let status_result = response.error_for_status_ref().map(|_| ());
    let text = response.text()?;

    if !status.is_success() {
        eprintln!("Status: {status}");
        eprintln!("Response: {text}");
    }

    status_result?;

    let result = serde_json::from_str::<SearchResponse>(&text)?;

    Ok(result)
}


#[derive(Debug, Deserialize)]
pub struct AddAssetResult {
    pub id: String,
    pub success: bool,
    pub error: Option<String>,
}

pub fn add_assets_to_album(
    client: &Client,
    server_url: &str,
    api_key: &str,
    album_id: &str,
    asset_ids: &[String],
) -> Result<Vec<AddAssetResult>, Box<dyn std::error::Error>> {
    if asset_ids.is_empty() {
        return Ok(Vec::new());
    }

    let url = format!(
        "{}/api/albums/{}/assets",
        server_url.trim_end_matches('/'),
        album_id,
    );

    let response = client
        .put(url)
        .header("x-api-key", api_key)
        .json(&json!({ "ids": asset_ids }))
        .send()?;

    let status = response.status();
    let status_result = response.error_for_status_ref().map(|_| ());
    let text = response.text()?;

    if !status.is_success() {
        eprintln!("Status: {status}");
        eprintln!("Response: {text}");
    }

    status_result?;

    let results = serde_json::from_str::<Vec<AddAssetResult>>(&text)?;
    Ok(results)
}