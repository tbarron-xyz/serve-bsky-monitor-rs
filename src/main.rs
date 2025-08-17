#![allow(non_snake_case)]

use std::sync::Arc;

use actix_files;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{AuthError, AuthResult, Authority, FromRequest, TokenSigner};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::web::{Form, Json};
use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use redis::Commands;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp_actix_web::StreamableHttpService;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, FromRequest)]
struct User {
    id: u32,
}

#[derive(Deserialize)]
struct LoginStruct {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(
    data: Json<LoginStruct>,
    token_signer: web::Data<TokenSigner<User, Ed25519>>,
) -> AuthResult<HttpResponse> {
    let mut redisPwd = redisCon().or(Err(
        HttpResponse::InternalServerError().body("redis con fail")
    ));
    if redisPwd.is_err() {
        return Ok(redisPwd.err().unwrap());
    }
    let pwd: Result<String> = redisPwd
        .unwrap()
        .get("admin_password")
        .or(Ok("admin".to_string()));
    let user = User { id: 1 };
    let inner = data.into_inner();
    if (inner.username == "admin" && inner.password == pwd.unwrap()) {
        return Ok(HttpResponse::Ok()
            .cookie(token_signer.create_access_cookie(&user)?)
            .cookie(token_signer.create_refresh_cookie(&user)?)
            .body("You are now logged in"));
    } else {
        return Ok(HttpResponse::BadRequest().body("No"));
    }
}

#[get("/logout")]
async fn logout() -> AuthResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("access_token", "")
                .path("/")
                .max_age(Duration::new(-1, 0))
                .finish(),
        )
        .cookie(
            Cookie::build("refresh_token", "")
                .path("/")
                .max_age(Duration::new(-1, 0))
                .finish(),
        )
        .body("You are now logged out"))
}

#[get("/hello")]
async fn hello(user: User) -> impl Responder {
    format!("Hello there, i see your user id is {}.", user.id)
}

mod mcpservice;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running httpserver");
    let KeyPair {
        pk: public_key,
        sk: secret_key,
    } = KeyPair::generate();
    HttpServer::new(move || {
        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Ed25519)
                    .build()
                    .expect(""),
            ))
            .verifying_key(public_key)
            .build()
            .expect("");

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
            .service(login)
            .service(logout)
            .service(actix_files::Files::new("/", "./static")) // this must come last as it's a catch-all
            .use_jwt(authority, web::scope("").service(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
