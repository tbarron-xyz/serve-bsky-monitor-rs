import { html, Component, render } from 'https://unpkg.com/htm/preact/standalone.module.js';
import { TinyJetstream as Jetstream } from 'https://cdn.jsdelivr.net/npm/mjbc@latest/tinyjetstream.js'

class Issue extends Component {
    render({ issue, i }) {
        const j = issue;
        return html`
            <div class="issue" id="issue${i}">
                <div class="time" id="time${i}"></div>
                <h1 class="newspaper-title" id="newspaper-title${i}">${j.newspaperName}</h1><hr/>
                <div class="coverStory">
                    <img class="photo coverPhoto" id="coverPhoto${i}"/><h2>${j.frontPageHeadline}</h2><p>${j.frontPageArticle}</p>
                </div>
                ${j.topics.filter((v,idx) => idx <= 3).map((v,idx) => html`
                    <div class="story">
                        <div class="gridContainer">
                            <img src="/grid${i}.jpg?${Date.now()}" class="gridimg grid${idx}" />
                        </div>
                        <h3>${v.headline}</h3>
                        <details>
                        <summary>${v.oneLineSummary}</summary>
                        <p class="storyP">${v.newsStoryFirstParagraph}<br/>${v.newsStorySecondParagraph}</p>
                        </details>

                        <div class="commentContainer">
                            <div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.gullibleComment}</div>
                            <div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.skepticalComment}</div>
                        </div>
                    </div>`)
                }
            </div>
    <hr/>`
    }
}

class MainPage extends Component {
    state = { news: [], refresh: true };

    componentDidMount() {
        this.fetchNews();
    }

    fetchNews = () => {
        if (this.state.refresh) {
            fetch("/news").then(res => {
                res.json().then(list => {
                    this.setState({ news: list });
                    if (JSON.stringify(list) != localStorage.getItem("state")) {
                        for (let i of list.keys()) {              
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

        setTimeout(this.fetchNews, 5000);
    }

    toggleRefresh = () => this.setState({ refresh: !this.state.refresh });

    render({}, { news = [] }) {
        return html`
            <div id="left-container">
                <div class="container" style>
                    <div id="leftad"></div>
                    <div id="rightad"></div>
                    <${TopNav} />
                    <div class="announcement">
                        <img style="height: 2.5em; float: left; margin-right: 15px;" src="https://mintlify.s3.us-west-1.amazonaws.com/mcp/mcp.png"/>
                        Browse via Model Context Protocol at 
                        <span style="margin-left:15px; font-weight: bold; font-family: monospace">https://skylines.news/mcp</span>
                    </div>
                    ${news.map((issue, i) => html`
                        <${Issue} issue=${issue} i=${i}></$>
                    `)}            
                    <div id="archive-link">Subscribe for access to the full historical archive.</div>
                    <i class="fa fa-refresh"></i><input checked type="checkbox" id="refresh"/>
                </div>
            </div>
            <div id="right-container">
                <div id="messages"></div>
                <i class="fa fa-refresh"></i><input type="checkbox" checked id="messagesRefresh" onInput={this.toggleRefresh}/>
            </div>`
    }
}

class LoginPage extends Component {
    render() {return html`<div class="container">
        <${TopNav} />
        <div>
            <input type="text" onInput="" />
            <input type="password" onInput="" />
            <input type="button">Login</input>
        </div>
    </div>`}
}

class TopNav extends Component {
    render() { return html`<div class="topnav">
                        <a href="#">front page</a> <!-- |  <a href="#newsroom">newsroom</a> --> | <a href="#login">login</a>
                    </div>`
    }
}

class NewsroomPage extends Component {
    render(){return html`Newsroom`}
}

import { HashRouter } from './preactStandaloneRouter.js';

// class HashRouter extends Component {
//     constructor() {
//         super();
//         this.state = { hash: window.location.hash };
//         addEventListener("hashchange", (event) => {
//             this.setState({ hash: window.location.hash });
//         });
//     }
//     render({ children, no_match }, { hash }) {
//         if (!Array.isArray(children)) { children = [ children ]; }  // preact children is just an object if only one child given
//         // const hash = window.location.hash;
//         const query = new URLSearchParams(window.location.search);
//         const el = children.find(x => `#${x.props.path}` == hash || x.props.path == hash);
//         return html`${el ? el : no_match()}`;
//     }
// }

class App extends Component {
    constructor() {
        super();
        this.state = { page: 
                window.location.hash == "#newsroom" ? "newsroom" :
                window.location.hash == "#login" ? "login" :
                "main"
            };
    }

    render({}, { page = "main" }) {
        return html`
        <${HashRouter}>
            <${MainPage} path=""/>
            <${NewsroomPage} path="newsroom"/>
            <${LoginPage} path="login"/>
        </$>`
    }
}
            
const jetstream = new Jetstream();

const last100 = [];
jetstream.onTweet = (event => {
    if (document.getElementById("messagesRefresh") && document.getElementById("messagesRefresh").checked) {
        const t = event.commit.record.text;
        last100.unshift(t);
        if (last100.length > 100) last100.pop();
        document.getElementById("messages").textContent = last100.join("\n");
    }
});

jetstream.start();

window.onload = () => {
    console.log("onload");
    localStorage.setItem("state","");
    render(html`<${App}/>`, document.getElementById("preact-app"));
};