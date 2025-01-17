use clap::Parser;
use reqwest::Error;
use serde::Deserialize;

#[derive(Parser)]
/// A simple CLI tool to fetch weather information.
struct Args {
    /// City name (e.g., "London")
    city: String,
    
    /// Unit system (metric/imperial)
    #[clap(short, long, default_value = "metric")]
    units: String,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temp_c: f32,
    humidity: u8,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
    forecast: Option<Forecast>,
}

#[derive(Deserialize)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize)]
struct ForecastDay {
    day: Day,
}

#[derive(Deserialize)]
struct Day {
    avgtemp_c: f32,
}

async fn fetch_weather(city: &str, _units: &str, api_key: &str) -> Result<WeatherResponse, Error> {
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, city
    );
    
    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let api_key = "dec2bc12080a40ba88a191953251701"; // Replace with your WeatherAPI key
    
    match fetch_weather(&args.city, &args.units, api_key).await {
        Ok(data) => {
            println!("Temperature: {}°C", data.current.temp_c);
            println!("Humidity: {}%", data.current.humidity);
            
            // Check if forecast data is present and print it
            if let Some(forecast) = data.forecast {
                if let Some(forecast_day) = forecast.forecastday.get(0) {
                    println!("Forecast Temperature: {}°C", forecast_day.day.avgtemp_c);
                } else {
                    println!("No forecast data available.");
                }
            } else {
                println!("No forecast data available.");
            }
        }
        Err(e) => eprintln!("Error fetching weather: {}", e),
    }
}

