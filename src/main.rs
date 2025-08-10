#![allow(non_snake_case)]

use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use redis::Commands;

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
    HttpResponse::Ok().content_type("text/html").body(
        r#"<html>
    <head>
        <title>skylines.news</title>
        <style>
            body { background: #333333;
                color: #cccccc;
                font-family: Helvetica Neue, Arial;
                margin: 0; padding: 0;
            }
            .newspaper-title { font-family: math; margin-bottom: 0; }
            #container { width: 1000px; max-width: 100%; min-width: 350px; padding: 10px; box-sizing: border-box; margin: auto; position: relative; }
            #left-container { display: inline-block; width: calc(100% - 250px); min-width: 350px; overflow: hidden; }
            #right-container { display: inline-block; width: 250px; float: right }
            .coverPhoto { width: 350px; float: right; }
            .photo { padding-left: 20px; }
            .coverStory { display: inline-block; font-size: 0.95em; padding-bottom: 10px; overflow: auto; max-width: 100%; }
            .commentContainer { padding-top: 13px }
            .halfcomment { display: inline-block; width: 50%; margin: 0; padding: 0; font-size: 0.9em; }
            #messages { height: 600px; overflow: hidden; }
            .time { float: right; }
            .storyP { font-size: 0.9em }
            .story { border-top: 1px dashed; padding-bottom: 10px; padding-top: 10px; overflow: auto; }
            .gridContainer { width: 223px;
                height: 223px;
                overflow: hidden;
                float: right; }
            .gridimg { width: 446px }
            .grid0 { }
            .grid1 { margin-left: -223px; }
            .grid2 { margin-top: -223px; }
            .grid3 { margin-left: -223px; margin-top: -223px; }
            .grid4 { display: none; }
            .grid5 { display: none; }
            #archive-link { padding: 10px;
                        margin: 0px 20px 20px 20px;
                        border: white solid 1px;
                        border-radius:  4px;
                        text-align: center;
                        background:  #444; }
            #leftad {
                background-image: url("/ad.jpg");
                width: 150px;
                height: 300px;
                position: absolute;
                margin-left: -165px;
                background-size: 300px;
            }
            #rightad {
                position: absolute;
                margin-right: -150px;
                width: 150px;
                height: 300px;
                background-image: url("/ad.jpg");
                background-size: 300px;
                background-position: 150px;
                right: 0px;
            }
            .issue {
                background: #222;
                padding: 20px;
                border-radius: 0px;
                margin-bottom: 20px;
                border: 1px solid #444;
            }
        </style>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/7.0.0/css/all.min.css" integrity="sha512-DxV+EoADOkOygM4IR9yXP8Sb2qwgidEmeqAEmDKIOfPRQZOWbXCzLC6vjbZyy0vPisbH2SyW27+ddLVCN+OMzQ==" crossorigin="anonymous" referrerpolicy="no-referrer" />
        <script>
class Jetstream {
  endpoint = "jetstream1.us-east.bsky.network";
  emitters = new Map();
        ws = null;

  get url() {
    const url = new URL(`wss://${this.endpoint}/subscribe`);

    for (const collection of this.emitters.keys()) {
      url.searchParams.append("wantedCollections", collection);
    }

    return url.toString();
  }

  constructor(options = {}) {
    this.endpoint = options.endpoint ?? this.endpoint;
  }

  #listen(collection, operation, listener) {
    const emitter = this.emitters.get(collection) || new EventTarget();
    this.emitters.set(collection, emitter);

    emitter.addEventListener(operation, listener);
  }

  onCreate(collection, listener) {
    this.#listen(collection, "create", listener);
  }

  onUpdate(collection, listener) {
    this.#listen(collection, "update", listener);
  }

  onDelete(collection, listener) {
    this.#listen(collection, "delete", listener);
  }

  start() {
    if (this.ws) this.ws.close();

    this.ws = new WebSocket(this.url);
    this.ws.onmessage = ev => {
      const data = JSON.parse(ev.data);
      if (data.kind !== "commit") return;

      const emitter = this.emitters.get(data.commit.collection);
      if (!emitter) return;
      emitter.dispatchEvent(new CustomEvent(data.commit.operation, { detail: data }));
    };
  }
}
const jetstream = new Jetstream();

const last100 = [];
jetstream.onCreate("app.bsky.feed.post", event => {
    if (document.getElementById("messagesRefresh").checked) {
        const t = event.detail.commit.record.text;
        last100.unshift(t);
        if (last100.length > 100) last100.pop();
        document.getElementById("messages").textContent = last100.join("\n");
    }
});

jetstream.start();

const refreshAll = () => {
    if (document.getElementById("refresh").checked) {
        fetch("/news").then(res => {
            res.json().then(list => {
                if (JSON.stringify(list) != localStorage.getItem("state")) {
                    for (let i of list.keys()) {
                        const j = list[i];
                        document.getElementById(`issue${i}`).innerHTML = `
    
    <div class="time" id="time${i}"></div>
    <h1 class="newspaper-title" id="newspaper-title${i}">${j.newspaperName}</h1><hr/>
    <div class="coverStory">
        <img class="photo coverPhoto" id="coverPhoto${i}"/><h2>${j.frontPageHeadline}</h2><p>${j.frontPageArticle}</p>
    </div>
    ${j.topics.filter((v,idx) => idx <= 3).map((v,idx) => `<div class="story">
        <div class="gridContainer"><img src="/grid${i}.jpg?${Date.now()}" class="gridimg grid${idx}"></div>
        <h3>${v.headline}</h3>
        <details>
            <summary>${v.oneLineSummary}</summary>
            <p class="storyP">${v.newsStoryFirstParagraph}<br/>${v.newsStorySecondParagraph}</p>
        </details>
        
        <div class="commentContainer"><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.gullibleComment}</div><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.skepticalComment}</div></div>
    </div>`).join("")}
    <hr/>
                        `;               
                        fetch(`/img${i}.jpg`).then(res => {
                            res.blob().then(blob => {
                                if (blob.size > 0) {
                                    const imageUrl = URL.createObjectURL(blob);
                                    document.imageUrl = imageUrl;
                                    document.getElementById(`coverPhoto${i}`).src = imageUrl;
                                }
                            });
                        });
                    }
                    localStorage.setItem("state", JSON.stringify(list));
                }
                fetch("/time").then(res => {
                    res.json().then(t => {
                        for (let idx of t.keys()) {
                            const i = parseInt(t[idx]);
                            const d = new Date(i).toLocaleString('en-US', { weekday: 'long', year: 'numeric', month: 'short', day: 'numeric', hour: 'numeric', minute: 'numeric' });
                            document.getElementById(`time${idx}`).textContent = d;
                        }
                    });
                });
            });
        });
    }

    setTimeout(refreshAll, 5000);
};
const refreshMessages = () => {
    fetch("/messages").then(res => {
        res.text().then(t => document.getElementById("messages").innerHTML = t);
    });
    //setTimeout(refreshMessages, 40); // disabling tweets auto update by default
};

window.onload = () => {
    localStorage.setItem("state","");
    refreshAll();
    // refreshMessages();
};
        </script>
    </head>
    <body>
        <div id="left-container">
            <div id="container" style>
                <div id="leftad"></div>
                <div id="rightad"></div>
                <div class="issue" id="issue0"></div><div class="issue" id="issue1"></div><div class="issue" id="issue2"></div><div class="issue" id="issue3"></div><div class="issue" id="issue4"></div>
                <div id="archive-link">Subscribe for access to the full historical archive.</div>
                <i class="fa fa-refresh"></i><input checked type="checkbox" id="refresh"/>
            </div>
        </div>
        <div id="right-container"><div id="messages"></div><i class="fa fa-refresh"></i><input type="checkbox" checked id="messagesRefresh"/></div>
    </body>
    </html>"#,
    )
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running httpserver");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(news)
            .service(img)
            .service(time)
            .service(grid)
            .service(fakeAd)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
