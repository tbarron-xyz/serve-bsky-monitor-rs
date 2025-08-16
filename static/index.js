import { html, Component, render } from 'https://unpkg.com/htm/preact/standalone.module.js';
import { Jetstream } from './jetstream.js';

class Issue extends Component {
    render(j) { return html`
<div class="time" id="time${j.i}"></div>
<h1 class="newspaper-title" id="newspaper-title${j.i}">${j.newspaperName}</h1><hr/>
<div class="coverStory">
    <img class="photo coverPhoto" id="coverPhoto${j.i}"/><h2>${j.frontPageHeadline}</h2><p>${j.frontPageArticle}</p>
</div>
${j.topics.filter((v,idx) => idx <= 3).map((v,idx) => html`
    <div class="story">
        <div class="gridContainer"><img src="/grid${i}.jpg?${Date.now()}" class="gridimg grid${idx}"></div>
        <h3>${v.headline}</h3>
        <details>
        <summary>${v.oneLineSummary}</summary>
        <p class="storyP">${v.newsStoryFirstParagraph}<br/>${v.newsStorySecondParagraph}</p>
        </details>

        <div class="commentContainer"><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.gullibleComment}</div><div class="halfcomment"><i class="fa fa-user" style="padding-right: 5px;"></i>${v.skepticalComment}</div></div>
    </div>`).join("")}
    <hr/>`
    }
}

class App extends Component {
addTodo() {
    const { todos = [] } = this.state;
    this.setState({ todos: todos.concat(`Item ${todos.length}`) });
}
render({ page }, { news = [] }) {
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
    <div class="app">
        <${Header} name="ToDo's (${page})" />
        <ul>
        ${news.map(todo => html`
            <${Issue} topics=${todo}>${todo}</$>
        `)}
        </ul>
        <button onClick=${() => this.addTodo()}>Add Todo</button>
        <${Footer}>footer content here<//>
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