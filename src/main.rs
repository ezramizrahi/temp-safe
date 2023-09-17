extern crate reqwest;
extern crate regex;

use std::env;
use regex::Regex;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a city name.");
        return Ok(());
    }
    let city = &args[1];

    let url = format!("https://wttr.in/{}?format=%25t", city);
    let resp = reqwest::get(&url).await?;
    let mut temperature_str = resp.text().await?;

    let re: Regex = Regex::new(r"([+-]?\d+)").unwrap();
    let cap = re.captures(&temperature_str).unwrap();
    temperature_str = String::from(&cap[1]);

    let temperature: i32 = temperature_str.parse().unwrap();

    println!("Current temperature: {}°C", temperature);

    match temperature {
        temp if temp >= 32 => println!("Life threatening temperature."),
        temp if temp >= 29 => println!("Unsafe temperature."),
        temp if temp >= 23 => println!("Potentially unsafe temperature."),
        _ => println!("Temperature is safe."),
    }

    Ok(())
}