
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Stock {
    name: String,
    price: f64,
}

struct AppState {
    client: HttpClient,
}

async fn get_stock_price(app_state: web::Data<AppState>, stock_name: web::Path<String>) -> impl Responder {
    let response = app_state.client.get(format!("https://www.nseindia.com/api/quote-equity?symbol={}", stock_name))
        .send()
        .await;

    match response {
        Ok(mut res) => {
            let stock: Stock = res.json().await.unwrap();
            HttpResponse::Ok().json(stock)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = HttpClient::new();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(AppState { client: client.clone() })) // Fixed code
            .route("/stock/{name}", web::get().to(get_stock_price))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}