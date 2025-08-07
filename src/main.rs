use actix_web::{error, get, App, Error, HttpResponse, HttpServer, Responder, Result};
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
        body {     background: #333333;
            color: #cccccc;
            font-family: Helvetica Neue, Arial;
        }
        .newspaper-title { font-family: math; margin-bottom: 0; }
        #container { width: 800px; float: left; display: inline-block; max-width: calc(100% - 300px); min-width: 350px; padding: 10px; }
        #right-container { width: 250px; float: right }
        .coverPhoto { width: 350px; float: right; }
        #coverStory { display: inline-block; font-size; 0.95em; padding-bottom: 10px; }
        .commentContainer { padding-top: 13px }
        .halfcomment { display: inline-block; width: 50%; margin: 0; padding: 0; font-size: 0.9em; }
        #messages { height: 600px; overflow: hidden; }
        .time { float: right; }
        .storyP { font-size: 0.9em }
        .story { border-top: 1px dashed; padding-bottom: 10px; }
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
        <img class="coverPhoto" id="coverPhoto${i}"/><h2>${j.frontPageHeadline}</h2><p>${j.frontPageArticle}</p>
    </div>
    ${j.topics.map(v => `<div class="story">
        <h3>${v.headline}</h3>
        <details>
            <summary>${v.oneLineSummary}</summary>
            <p class="storyP">${v.newsStoryFirstParagraph}<br/>${v.newsStorySecondParagraph}</p>
        </details>
        
        <div class="commentContainer"><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.gullibleComment}</div><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.skepticalComment}</div></div>
    </div>`).join("")}
    <hr/><br/>
                        `;
                    }
                    localStorage.setItem("state", JSON.stringify(list));
                }

                // const j = list[0];
                // if (JSON.stringify(j) != localStorage.getItem("state")) {
                //     // document.getElementById("news").innerHTML = JSON.stringify(j);
                //     console.log(j);
                //     document.getElementById("newspaper-title").textContent = j.newspaperName;
                //     for (let i of [0,1,2,3,4]) {
                //         let v = j.topics[i];
                //         try {
                //             document.getElementById(`story${i+1}`).innerHTML = `<h3>${v.headline}</h3>
                //             <details>
                //                 <summary>${v.oneLineSummary}</summary>
                //                 <p class="storyP">${v.newsStoryFirstParagraph}<br/>${v.newsStorySecondParagraph}</p>
                //             </details>
                            
                //             <div class="commentContainer"><div class="halfcomment"><i class="fa fa-user"  style="padding-right: 5px;"></i>${v.gullibleComment}</div><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.skepticalComment}</div></div>`; 
                //         } catch (e) { }
                //     }
                //     document.getElementById("coverStory").innerHTML = `<img id="coverPhoto"/><h2>${j.frontPageHeadline}</h2><p>${j.frontPageArticle}</p>`;
                //     document.getElementById("coverPhoto").src = document.imageUrl;
                // }

                fetch("/img.jpg").then(res => {
                    res.blob().then(blob => {
                        const imageUrl = URL.createObjectURL(blob);
                        document.imageUrl = imageUrl;
                        document.getElementById("coverPhoto0").src = imageUrl;
                    });
                });
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
        <div id="container" style>
            <div id="issue0"></div><div id="issue1"></div><div id="issue2"></div><div id="issue3"></div><div id="issue4"></div>
            <!--<div id="time"></div>
            <h1 id="newspaper-title"></h1><hr/>
            <div id="coverStory"></div>
            <div class="story" id="story1"></div><div class="story" id="story2"></div><div class="story" id="story3"></div><div class="story" id="story4"></div><div class="story" id="story5"></div>-->
            <i class="fa fa-refresh"></i><input checked type="checkbox" id="refresh"/>
        </div>
        <div id="right-container"><div id="messages"></div><i class="fa fa-refresh"></i><input type="checkbox" checked id="messagesRefresh"/></div>
    </body>
    </html>"#,
    )
    //        Maybe later... <script src="https://cdn.tailwindcss.com/3.4.16"></script>
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

// #[get("/summary")]
// async fn summary() -> Result<String> {
//     let con = redisCon();
//     let result = con?.get("currentSummary").or(Ok("".to_string()));
//     return result;
// }

// #[get("/subtopics")]
// async fn subtopics() -> Result<String> {
//     let con = redisCon();
//     let result = con?.get("subtopics").or(Ok("".to_string()));
//     return result;
// }

// #[get("/messages")]
// async fn messagesList() -> Result<String> {
//     return Ok("".to_string()); // they can get messages themselves from jetstream
//     let con = redisCon();
//     let result: Result<Vec<String>> = con?.lrange("messagesList", 0, 30).or(Ok(vec![]));
//     println!("getting result");
//     return result
//         .inspect(|x| {
//             println!("returning {}", x.join(" "));
//         })
//         .map(|x| "[" + x.join(",") + "]");
// }

// #[get("/trends")]
// async fn trends() -> Result<String> {
//     let con = redisCon();
//     let result = con?.get("currentTrends").or(Ok("".to_string()));

//     return result;
// }

#[get("/news")]
async fn news() -> Result<String> {
    let con = redisCon();
    let result: Result<Vec<String>> = con?.lrange("newsList", 0, 5).or(Ok(vec![]));
    // println!("getting result");
    return result
        .inspect(|x| {
            // println!("returning {}", x.join(" "));
        })
        .map(|x| format!("[{}]", x.join(",")));

    // let result = con?.get("newsTopics").or(Ok("".to_string()));

    // return result;
}

#[get("/time")]
async fn time() -> Result<String> {
    let con = redisCon();
    let result: Result<Vec<String>> = con?.lrange("timeList", 0, 5).or(Ok(vec![]));
    // println!("getting result");
    return result
        .inspect(|x| {
            // println!("returning {}", x.join(" "));
        })
        .map(|x| format!("[{}]", x.join(",")));
    // let time = con?.get("newsTopicsTime").or(Ok(0.to_string()));

    // return time;
}

#[get("/img.jpg")]
async fn img() -> Result<HttpResponse> {
    let con = redisCon();
    let result = con?
        .get("img")
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
            // .service(summary)
            // .service(subtopics)
            // .service(messagesList)
            // .service(trends)
            .service(news)
            .service(img)
            .service(time)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
