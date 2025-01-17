use clap::Parser;
use reqwest::Error;
use serde_json::Value;
use colored::*;  // Import the colored crate

#[derive(Parser)]
/// A simple CLI tool to fetch weather information.
struct Args {
    /// City name (e.g., "London")
    #[clap(short, long, default_value = "Budapest")]
    city: String,
    
    /// Unit system (metric/imperial)
    #[clap(short, long, default_value = "metric")]
    units: String,
}

async fn fetch_weather(city: &str, _units: &str, api_key: &str) -> Result<Value, Error> {
    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=no",  // Requesting forecast for 3 days
        api_key, city
    );
    
    // Fetch the weather data as raw JSON
    let response = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let api_key = "dec2bc12080a40ba88a191953251701"; // Replace with your WeatherAPI key
    
    match fetch_weather(&args.city, &args.units, api_key).await {
        Ok(data) => {
            // Use colors to enhance the output
            let current = &data["current"];
            println!("{}", "Current Weather Information:".bold().green());
            println!("Temperature: {}°C", current["temp_c"].to_string().red());
            println!("Feels like: {}°C", current["feelslike_c"].to_string().cyan());
            println!("Humidity: {}", current["humidity"].to_string().purple());
            println!("Condition: {}", current["condition"]["text"].to_string().yellow());
            println!("Cloud cover: {}", current["cloud"].to_string().blue());
            println!("UV: {}", current["uv"].to_string().white());
            println!("Wind speed (kph): {}", current["wind_kph"].to_string().blue());
            println!("Wind direction: {}", current["wind_dir"].to_string().green());
            println!("Pressure (mb): {}", current["pressure_mb"].to_string().bright_green());
            println!("Last updated: {}", current["last_updated"].to_string().bright_white());
            
            if let Some(forecast) = data["forecast"]["forecastday"].as_array() {
                for (i, forecast_day) in forecast.iter().enumerate() {
                    println!("\n{}", format!("Forecast for day {}", i + 1).bold().magenta());
                    println!("Date: {}", forecast_day["date"].to_string().bright_cyan());
                    println!("Avg Temperature: {}°C", forecast_day["day"]["avgtemp_c"].to_string().yellow());
                    println!("Max Temperature: {}°C", forecast_day["day"]["maxtemp_c"].to_string().red());
                    println!("Min Temperature: {}°C", forecast_day["day"]["mintemp_c"].to_string().blue());
                    println!("Condition: {}", forecast_day["day"]["condition"]["text"].to_string().green());
                    println!("Chance of rain: {}", forecast_day["day"]["daily_chance_of_rain"].to_string().bright_blue());
                    println!("Chance of snow: {}", forecast_day["day"]["daily_chance_of_snow"].to_string().bright_purple());
                    println!("Sunrise: {}", forecast_day["astro"]["sunrise"].to_string().bright_yellow());
                    println!("Sunset: {}", forecast_day["astro"]["sunset"].to_string().bright_yellow());
                    println!("Moonphase: {}", forecast_day["astro"]["moon_phase"].to_string().bright_yellow());
                    println!("Moonrise: {}", forecast_day["astro"]["moonrise"].to_string().bright_yellow());
                    println!("Moonset: {}", forecast_day["astro"]["moonset"].to_string().bright_yellow());
                }
            }
        }
        Err(e) => eprintln!("{}", format!("Error fetching weather: {}", e).red()),
    }
}

