import { html, Component, render } from 'https://unpkg.com/htm/preact/standalone.module.js';
import { Jetstream } from './jetstream.js';

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

class App extends Component {
addTodo() {
    const { todos = [] } = this.state;
    this.setState({ todos: todos.concat(`Item ${todos.length}`) });
}
render({ page }, { news: issues = [] }) {
    fetch("/news").then(res => {
        res.json().then(list => {
            this.setState({ news: list });
            // if (JSON.stringify(list) != localStorage.getItem("state")) {
            //     for (let i of list.keys()) {
            //         const j = list[i];
            //         document.getElementById(`issue${i}`).innerHTML = ;               
            //         fetch(`/img${i}.jpg`).then(res => {
            //             res.blob().then(blob => {
            //                 if (blob.size > 0) {
            //                     const imageUrl = URL.createObjectURL(blob);
            //                     document.imageUrl = imageUrl;
            //                     document.getElementById(`coverPhoto${i}`).src = imageUrl;
            //                 }
            //             });
            //         });
            //     }
            //     localStorage.setItem("state", JSON.stringify(list));
            // }
            // fetch("/time").then(res => {
            //     res.json().then(t => {
            //         for (let idx of t.keys()) {
            //             const i = parseInt(t[idx]);
            //             const d = new Date(i).toLocaleString('en-US', { weekday: 'long', year: 'numeric', month: 'short', day: 'numeric', hour: 'numeric', minute: 'numeric' });
            //             document.getElementById(`time${idx}`).textContent = d;
            //         }
            //     });
            // });
        });
    });
    // fetch(x,x=>{
    //     this.setState()
    // });
    return html`
    <div id="left-container">
        <div id="container" style>
            <div id="leftad"></div>
            <div id="rightad"></div>
            <div class="announcement">
                <img style="height: 2.5em; float: left; margin-right: 15px;" src="https://mintlify.s3.us-west-1.amazonaws.com/mcp/mcp.png"/>Browse via Model Context Protocol at <span style="margin-left:15px; font-weight: bold; font-family: monospace">https://skylines.news/mcp</span>
            </div>
    ${issues.map((issue, i) => html`
        <${Issue} issue=${issue} i=${i}></$>
    `)}            <div id="archive-link">Subscribe for access to the full historical archive.</div>
            <i class="fa fa-refresh"></i><input checked type="checkbox" id="refresh"/>
        </div>
    </div>
    <div id="right-container">
        <div id="messages"></div>
        <i class="fa fa-refresh"></i><input type="checkbox" checked id="messagesRefresh"/>
    </div>
    `;
}
}

const Header = ({ name }) => html`<h1>${name} List</h1>`

const Footer = props => html`<footer ...${props} />`

render(html`<${App} page="All" />`, document.getElementById("preact-app"));

            
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
        
    }

    setTimeout(refreshAll, 5000);
};


window.onload = () => {
    localStorage.setItem("state","");
    refreshAll();
};