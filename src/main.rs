mod api_immich;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    server_url: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Rule {
    album_id: String,
    person_id: String,
    api_key: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_created_at: Option<String>,
}

use chrono::{DateTime, Duration, SecondsFormat};

fn subtract_one_day(timestamp: Option<String>) -> Option<String> {
    let value = timestamp?;
    let date = DateTime::parse_from_rfc3339(&value).ok()?;
    let previous = date.checked_sub_signed(Duration::days(1))?;

    Some(previous.to_rfc3339_opts(SecondsFormat::Millis, true))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let yaml = std::fs::read_to_string("config.yaml")?;
    let mut config: Config = serde_yaml_ng::from_str(&yaml)?;

    println!("Server: {}", config.server_url);

    for rule in &config.rules {
        println!("Start {:?} Person {} → album {}", rule.last_created_at, rule.person_id, rule.album_id);
    }

    let client = reqwest::blocking::Client::new();



    for rule in &mut config.rules {
        let mut cursor: Option<String> = None;
        let mut latest = rule.last_created_at.clone();
        let last_created_at: Option<String> = subtract_one_day(rule.last_created_at.clone());
        loop {
            let response = api_immich::search_assets(
                &client,
                &config.server_url,
                &rule.api_key,
                &rule.person_id,
                last_created_at.as_deref(),
                cursor.as_deref(),
            )?;

            for asset in &response.assets.items {
                println!("{}", asset.id);

                if latest.as_ref().is_none_or(|t| {
                    asset.created_at.as_str() > t.as_str()
                }) {
                    latest = Some(asset.created_at.clone());
                }
            }

            let ids: Vec<String> = response.assets.items
                .iter()
                .map(|asset| asset.id.clone())
                .collect();

            let results = api_immich::add_assets_to_album(
                &client,
                &config.server_url,
                &rule.api_key,
                &rule.album_id,
                &ids,
            )?;

            for result in results {
                if !result.success && result.error.as_deref() != Some("duplicate") {
                    return Err(format!(
                        "Failed to add asset {}: {:?}",
                        result.id, result.error
                    ).into());
                }
            }

            cursor = response.assets.next_cursor;

            if cursor.is_none() {
                break;
            }
        }
        rule.last_created_at = latest;
    }

    // Serialize, then replace the config using a temporary file.
    let options = serde_saphyr::ser_options! {
        indent_step: 4,
        compact_list_indent: false,
    };
    let updated_yaml = serde_saphyr::to_string_with_options(&config, options)?;
    // let updated_yaml = serde_yaml_ng::to_string(&config)?;
    std::fs::write("config.yaml.tmp", updated_yaml)?;
    std::fs::rename("config.yaml.tmp", "config.yaml")?;

    Ok(())
}
