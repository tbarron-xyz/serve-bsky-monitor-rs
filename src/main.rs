#![allow(non_snake_case)]

use std::sync::Arc;

use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use actix_files;
use redis::Commands;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp_actix_web::StreamableHttpService;

use crate::mcpservice::NewsMcpService;

fn redisCon() -> Result<redis::Connection, Error> {
    let client = redis::Client::open("redis://127.0.0.1/")
        .map_err(|_err| error::ErrorInternalServerError("client fail"));
    let con = client?
        .get_connection()
        .map_err(|_err| error::ErrorInternalServerError("con fail"));
    return con;
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
    //        Maybe later... <script src="https://cdn.tailwindcss.com/3.4.16"></script>
}

#[get("/news")]
async fn news() -> Result<String> {
    let con = redisCon();
    let result: Result<Vec<String>> = con?.lrange("newsList", 0, 4).or(Ok(vec![]));
    return result
        .inspect(|_x| {
            // println!("returning {}", x.join(" "));
        })
        .map(|x| format!("[{}]", x.join(",")));
}

#[get("/sections")]
async fn sections() -> Result<String> {
    let con = redisCon();
    let result: Result<String> = con?.get("beats").or(Ok("".to_string()));
    return result;
}

#[get("/time")]
async fn time() -> Result<String> {
    let con = redisCon();
    let result: Result<Vec<String>> = con?.lrange("timeList", 0, 4).or(Ok(vec![]));
    return result
        .inspect(|_x| {
            // println!("returning {}", x.join(" "));
        })
        .map(|x| format!("[{}]", x.join(",")));
}

#[get("/img.jpg")]
async fn img0() -> Result<HttpResponse> {
    let con = redisCon();
    let result = con?
        .get("img")
        .map(|x: Vec<u8>| HttpResponse::Ok().content_type("image/jpeg").body(x))
        .or(Ok(HttpResponse::Ok().body("".to_string())));

    return result;
}

#[get("/grid{id}.jpg")]
async fn grid(info: web::Path<(u32)>) -> Result<HttpResponse> {
    let info = info.into_inner();
    let id = info;
    let con = redisCon();
    let result: Result<Vec<u8>> = con?.lindex("imgGridList", id as isize).or(Ok(vec![]));
    return result
        .map(|x: Vec<u8>| HttpResponse::Ok().content_type("image/jpeg").body(x))
        .or(Ok(HttpResponse::Ok().body("".to_string())));
}

#[get("/img{id}.jpg")]
async fn img(info: web::Path<(u32)>) -> Result<HttpResponse> {
    let info = info.into_inner();
    let id = info;
    let con = redisCon();
    let result: Result<Vec<u8>> = con?.lindex("imgList", id as isize).or(Ok(vec![]));
    return result
        .map(|x: Vec<u8>| HttpResponse::Ok().content_type("image/jpeg").body(x))
        .or(Ok(HttpResponse::Ok().body("".to_string())));
}

#[get("/ad.jpg")]
async fn fakeAd() -> Result<HttpResponse> {
    let con = redisCon();
    let result = con?
        .get("fakeAd")
        .map(|x: Vec<u8>| HttpResponse::Ok().content_type("image/jpeg").body(x))
        .or(Ok(HttpResponse::Ok().body("".to_string())));

    return result;
}

mod mcpservice;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running httpserver");

    HttpServer::new(|| {
        let http_service = Arc::new(StreamableHttpService::new(
            || Ok(NewsMcpService::new()),
            LocalSessionManager::default().into(),
            Default::default(),
        ));
        let http_scope = StreamableHttpService::scope(http_service);
        App::new()
            .service(index)
            .service(news)
            .service(img)
            .service(time)
            .service(grid)
            .service(fakeAd)
            .service(sections)
            .service(web::scope("/mcp").service(http_scope))
            .service(actix_files::Files::new("/", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
