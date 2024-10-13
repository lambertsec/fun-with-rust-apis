extern crate actix_web;
mod service;
use actix_web::{error, Error, get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("current folder {}", std::env::current_dir().unwrap().display());
    HttpServer::new(|| {
        App::new().route("/",web::get().to(hello))
            .service(ageapi)
            .service(time)
            .service(file_return)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize)]
struct file{
    file_name: String
}

#[get("/age/{year}")]
async fn ageapi(year: web::Path<String>) -> impl Responder {
    println!("Age request received");
    let string_year :String = year.into_inner();
    HttpResponse::Ok().body(
        format!("You are {} years old", service::calculate_age(string_year)))
}

#[post("/time")]
async fn time() -> impl Responder {
    HttpResponse::Ok().body(service::get_time().await)
}

#[post("/file")]
async fn file_return(mut payload: web::Payload) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE{
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk)
    }
    let obj = serde_json::from_slice::<file>(&body)?;
    Ok(HttpResponse::Ok().json(service::read_file(obj.file_name)))
}