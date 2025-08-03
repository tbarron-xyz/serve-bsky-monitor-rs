use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer, Responder, Result};
// use redis::Client;
use redis::Commands;

fn redisCon() -> Result<redis::Connection, Error> {
    let client = redis::Client::open("redis://127.0.0.1/")
        .map_err(|_err| error::ErrorInternalServerError("client fail"));
    let mut con = client?
        .get_connection()
        .map_err(|_err| error::ErrorInternalServerError("con fail"));
    return con;
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<html>
    <head>
    <style>
        #container { width: 600px }
    </style>
      <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
        <script>
const refreshAll = () => {
    fetch("/summary").then(res => {
        console.log("fetched summary");
        res.text().then(t => document.getElementById("currentSummary").textContent = t);
    });
    fetch("/subtopics").then(res => {
        console.log("fetched subtopics");
        res.json().then(t => {
            const technology = t.technology;
            const values = Object.values(t).flatMap(x => x).map(x => x.name).join("<br/>");
            document.getElementById("subtopics").innerHTML = `<h2>Topics:</h2>${values}`;
        });
    });

    setTimeout(refreshAll, 5000);
};
const refreshMessages = () => {
        fetch("/messages").then(res => {
        console.log("fetched messages");
        res.text().then(t => document.getElementById("messages").textContent = t);
    });
    setTimeout(refreshMessages, 40);
    };

window.onload = () => {
    refreshAll();
    refreshMessages();
};
        </script>
    </head>
    <body>
        <div id="container">
        <h1>Bluesky Monitor</h1>
        <div id="currentSummary"></div>
        <div id="subtopics"></div>
        <h2>Messages:</h2> <div id="messages"></div>
        </div>
    </body>
    </html>"#,
    ) // todo proper html response
}

#[get("/summary")]
async fn summary() -> Result<String> {
    let con = redisCon();
    let result = con?.get("currentSummary").or(Ok("".to_string()));

    return result;
}

#[get("/subtopics")]
async fn subtopics() -> Result<String> {
    let con = redisCon();
    let result2 = con?.get("subtopics").or(Ok("".to_string()));
    return result2;
}

#[get("/messages")]
async fn messagesList() -> Result<String> {
    let con = redisCon();
    let result: Result<Vec<String>> = con?.lrange("messagesList", 0, 30).or(Ok(vec![]));
    println!("getting result");
    return result
        .inspect(|x| {
            println!("returning {}", x.join(" "));
        })
        .map(|x| x.join(" "));
    // return result;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running httpserver");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(summary)
            .service(subtopics)
            .service(messagesList)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
