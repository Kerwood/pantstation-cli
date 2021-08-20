#[macro_use]
extern crate prettytable;
use colored::*;
use pantstation::*;
use prettytable::Table;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pantstation")]
enum Pantstation {
    #[structopt(name = "list", about = "List all pantstations.")]
    List {},

    #[structopt(name = "get", about = "Get information on specific pant station.")]
    Get {
        #[structopt(help = "City of pant station")]
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Pantstation::from_args() {
        Pantstation::List {} => match get_station_data().await {
            Ok(x) => {
                let mut table = Table::new();
                table.set_format(table_format());
                table.set_titles(row!(b -> "Pant Station", b -> "Address", b -> "Status", b -> "Type", b -> "Opening Hours"));

                for station in x.iter() {
                    table.add_row(row!(
                        &station.location.city,
                        format!(
                            "{} {}",
                            &station.location.street_name, &station.location.street_number
                        ),
                        format!("{}", color_status(&station.operational_status)),
                        format_type(&station.station_type),
                        strip_trailing_newline(&station.opening_hours),
                    ));
                }
                println!();
                table.printstd();
            }
            Err(error) => Err(error)?,
        },

        Pantstation::Get { name } => match get_station_data().await {
            Ok(stations) => {
                let filtered_stations: Vec<&Station> = stations
                    .iter()
                    .filter(|x| x.location.city.to_lowercase() == name.to_lowercase())
                    .collect();

                if filtered_stations.len() == 0 {
                    println!("No city found by that name..");
                    process::exit(1);
                }

                for station in filtered_stations.iter() {
                    let location_string = format!(
                        "{}, {} {}, {}",
                        station.location.city,
                        station.location.street_name,
                        station.location.street_number,
                        station.location.post_code
                    );
                    let google_maps = format!(
                        "https://www.google.com/maps/search/?api=1&query={}%2C{}\n",
                        station.location.lat, station.location.lng
                    );

                    println!("\n{}\n", location_string.bold().underline());

                    println!(
                        "{} {}\n{} {}",
                        "Status:".bold(),
                        color_status(&station.operational_status),
                        "Station Type:".bold(),
                        format_type(&station.station_type)
                    );
                    println!("{} {}", "Opening Hours:".bold(), station.opening_hours);
                    println!("{} {}", "Location:".bold(), google_maps);

                    println!("{}", "Description".bold());
                    println!("{}", strip_paragraf_chars(&station.short_description));

                    if station.operational_status == "custom" {
                        println!("\n{}", "Operational Status".bold());
                        println!("{}", strip_paragraf_chars(&station.operational_status_text).red());
                    }

                    if station.important_notification && station.important_notification_text != None
                    {
                        println!("\n{}", "Important Notification".bold());
                        println!(
                            "{}",
                            strip_paragraf_chars(
                                &station.important_notification_text.as_ref().unwrap()
                            )
                            .red()
                        );
                    }
                    println!();
                }
            }
            Err(error) => Err(error)?,
        },
    }
    Ok(())
}
