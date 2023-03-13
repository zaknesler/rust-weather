mod model;
mod weather;

use clap::{arg, crate_version, Command};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn cli() -> Command {
    Command::new("cli")
        .about("CLI demo app")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .version(crate_version!())
        .subcommand(
            Command::new("weather")
                .arg(arg!(<LAT>).help("Latitude").required(true))
                .arg(arg!(<LONG>).help("Longitude").required(true))
                .about("Fetch and print the weather"),
        )
        .arg(
            arg!(--key <KEY>)
                .short('k')
                .help("API key for OpenWeatherMap")
                .number_of_values(1)
                .required(true),
        )
        .arg(
            arg!(--log <LEVEL>)
                .short('l')
                .help("Override log level")
                .long_help("Override log level. Valid values are: trace, debug, info, warn, error")
                .number_of_values(1)
                .default_value("info"),
        )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = cli().get_matches();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(match matches.get_one::<String>("log").unwrap().as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let api_key = matches.get_one::<String>("key").unwrap();

    match matches.subcommand() {
        Some(("weather", sub_matches)) => {
            let mut client = weather::WeatherClient::new(api_key.to_string());
            client.init_client()?;

            let lat = sub_matches
                .get_one::<String>("LAT")
                .unwrap()
                .parse::<f64>()
                .expect("Latitude must be a number");
            let long = sub_matches
                .get_one::<String>("LONG")
                .unwrap()
                .parse::<f64>()
                .expect("Longitude must be a number");

            let data = client.by_lat_long(lat, long).await?;

            let city = data.city_name.unwrap_or_default();
            let weather = data.weather.first().map(|w| &w.main).unwrap();
            let emoji = data
                .weather
                .first()
                .map(|w| w.get_emoji())
                .flatten()
                .unwrap_or("");
            println!("Weather in {}: {} {}", city, weather, emoji);
        }
        _ => {}
    }

    Ok(())
}
