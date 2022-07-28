use colored::*;
use prettytable::format;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use voca_rs::Voca;

pub type Root = Vec<RootElement>;

#[derive(Serialize, Deserialize)]
pub struct RootElement {
    #[serde(rename = "acf")]
    station: Station,
}

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
    pub street_number: Option<String>,
    pub street_name: String,
    pub city: String,
    pub post_code: String,
    pub country: String,
    pub country_short: String,
}

impl Station {
    pub fn clean(self) -> Station {
        let trimmer = |string: String| -> String {
            string
                ._strip_tags()
                .replace("&nbsp;", "")
                .trim()
                .to_string()
        };

        Station {
            location: self.location,
            opening_hours: trimmer(self.opening_hours),
            operational_status: trimmer(self.operational_status),
            operational_status_text: trimmer(self.operational_status_text),
            station_type: trimmer(self.station_type),
            short_description: trimmer(self.short_description),
            important_notification: self.important_notification,
            important_notification_text: match self.important_notification_text {
                Option::Some(s) => Some(trimmer(s)),
                Option::None => None,
            },
        }
    }
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

    let root: Root = serde_json::from_str(&res)?;

    for root_element in root.into_iter() {
        stations.push(root_element.station.clean());
    }
    Ok(stations)
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
        "app" => "App",
        &_ => "",
    }
}
