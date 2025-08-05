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
        #container { width: 600px; float: left }
        #right-container { width: 400px; float: right }
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
    const t = event.detail.commit.record.text;
    last100.unshift(t);
    if (last100.length > 100) last100.pop();
    document.getElementById("messages").textContent = last100.join("\n");
    // console.log(t);
});

jetstream.start();

const refreshAll = () => {
    fetch("/news").then(res => {
        res.json().then(j => {
            // document.getElementById("news").innerHTML = JSON.stringify(j);
            console.log(j);
            document.getElementById("newspaper-title").textContent = j.newspaperName;
            for (let i of [0,1,2,3,4]) {
                let v = j.topics[i];
                try {
                    document.getElementById(`story${i+1}`).innerHTML = `<h3>${v.headline}</h3>${v.newsStoryFirstParagraph}<br/>${v.newsStorySecondParagraph}
                    <div><i class="fa fa-user"></i>${v.gullibleComment}</di><div><i class="fa fa-user"></i>${v.skepticalComment}</div>`; 
                } catch (e) { }
            }
            document.getElementById("coverStory").innerHTML = `<h2>${j.frontPageHeadline}</h2><p>${j.frontPageArticle}</p>`;
        });
    });

    setTimeout(refreshAll, 5000);
};
const refreshMessages = () => {
    fetch("/messages").then(res => {
        // console.log("fetched messages");
        res.text().then(t => document.getElementById("messages").innerHTML = t);
    });
    //setTimeout(refreshMessages, 40); // disabling tweets auto update by default
};

window.onload = () => {
    refreshAll();
    refreshMessages();
};
        </script>
    </head>
    <body>
        <div id="container" style>
            <h1 id="newspaper-title">Bluesky Monitor</h1>
            <div id="coverStory"></div>
            <div id="story1"></div><div id="story2"></div><div id="story3"></div><div id="story4"></div><div id="story5"></div>
        </div>
        <div id="right-container"><h2>Messages:</h2><div id="messages"></div></div>
    </body>
    </html>"#,
    )
    // fetch("/summary").then(res => {
    //     console.log("fetched summary");
    //     res.text().then(t => document.getElementById("currentSummary").textContent = t);
    // });
    // fetch("/subtopics").then(res => {
    //     console.log("fetched subtopics");
    //     res.json().then(t => {
    //         const technology = t.technology;
    //         const values = Object.values(t).flatMap(x => x).map(x => x.name).join("<br/>");
    //         document.getElementById("subtopics").innerHTML = `${values}`;
    //     });
    // });
    // fetch("/trends").then(res => {
    //     console.log("fetched trends");
    //     res.json().then(t => {
    //         const values = t.join("<br/>");
    //         document.getElementById("trends").innerHTML = `${values}`;
    //     });
    // });
    // <div id="currentSummary"></div>
    // <h2>Trends:</h2> <div id="trends"></div>
    // <h2>Topics:</h2><div id="subtopics"></div>
    // <div id="news"></div>
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
    let result = con?.get("subtopics").or(Ok("".to_string()));
    return result;
}

#[get("/messages")]
async fn messagesList() -> Result<String> {
    return Ok("".to_string()); // they can get messages themselves from jetstream
    let con = redisCon();
    let result: Result<Vec<String>> = con?.lrange("messagesList", 0, 30).or(Ok(vec![]));
    println!("getting result");
    return result
        .inspect(|x| {
            println!("returning {}", x.join(" "));
        })
        .map(|x| x.join("<br/>"));
}

#[get("/trends")]
async fn trends() -> Result<String> {
    let con = redisCon();
    let result = con?.get("currentTrends").or(Ok("".to_string()));

    return result;
}

#[get("/news")]
async fn news() -> Result<String> {
    let con = redisCon();
    let result = con?.get("newsTopics").or(Ok("".to_string()));

    return result;
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
            .service(trends)
            .service(news)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
