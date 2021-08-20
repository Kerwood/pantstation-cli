use colored::*;
use prettytable::format;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub location: Location,
    pub opening_hours: String,
    pub operational_status: String,
    pub operational_status_text: String,
    pub station_type: String,
    pub short_description: String,
    pub important_notification: bool,
    pub important_notification_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub address: String,
    pub lat: f32,
    pub lng: f32,
    pub name: String,
    pub street_number: String,
    pub street_name: String,
    pub city: String,
    pub post_code: String,
    pub country: String,
    pub country_short: String,
}

pub async fn get_station_data() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let mut stations: Vec<Station> = Vec::new();
    let params = [("action", "getMapData")];
    let client = reqwest::Client::new();
    let res = client
        .post("https://danskretursystem.dk/wp/wp-admin/admin-ajax.php")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let acf: Vec<Value> = serde_json::from_str(&res)?;

    for item in acf.iter() {
        let station = item["acf"].clone();
        stations.push(serde_json::from_value(station)?);
    }
    Ok(stations)
}

pub fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("<br />\r\n")
        .or(input.strip_suffix("\r\n"))
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

pub fn strip_paragraf_chars(input: &str) -> &str {
    input
        .strip_suffix("</p>\n")
        .unwrap_or(input)
        .strip_prefix("<p>")
        .unwrap_or(input)
}

pub fn color_status(input: &str) -> ColoredString {
    match input {
        "normal" => "Normal".green(),
        "breakdown" => "Breakdown".red(),
        "full" => "Full".cyan(),
        "custom" => "Custom".yellow(),
        _ => input.yellow(),
    }
}

pub fn table_format() -> format::TableFormat {
    format::FormatBuilder::new()
        .column_separator(' ')
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('-', ' ', ' ', ' '),
        )
        .padding(1, 1)
        .build()
}

pub fn format_type(status: &str) -> &str {
    match status {
        "dropngo" => "Drop'n Go",
        "sack" => "Sack",
        "seperateglas" => "Seperate glass",
        &_ => "unknown_type",
    }
}
