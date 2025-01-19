use actix_web::{web, App, HttpServer, HttpResponse, Responder};  
use serde::Deserialize;  
use std::env;  
use dotenv::dotenv;  
  
#[derive(Deserialize)]  
struct WeatherQuery {  
    city: String,  
}  
  
async fn get_weather(query: web::Query<WeatherQuery>) -> impl Responder {  
    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("API key not set");  
    let city = &query.city;  
  
    let url = format!(  
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",  
        city, api_key  
    );  
  
    let response = reqwest::get(&url).await;  
  
    match response {  
        Ok(resp) => {  
            println!("Response status: {}", resp.status());
            if resp.status().is_success() {  
                let weather_data: serde_json::Value = resp.json().await.unwrap();  
                HttpResponse::Ok().json(weather_data)  
            } else {  
                HttpResponse::BadRequest().body("Failed to fetch weather data")  
            }  
        }  
        Err(_) => HttpResponse::InternalServerError().body("Error making request"),  
    }  
}  
  
#[actix_web::main]  
async fn main() -> std::io::Result<()> {  
    dotenv().ok(); // Memuat variabel dari .env  
  
    HttpServer::new(|| {  
        App::new()  
            .route("/weather", web::get().to(get_weather))  
    })  
    .bind("127.0.0.1:8080")?  
    .run()  
    .await  
}  
