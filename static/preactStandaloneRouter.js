import { html, Component } from 'https://unpkg.com/htm/preact/standalone.module.js';

export class HashRouter extends Component {
    constructor() {
        super();
        this.state = { hash: window.location.hash };
        addEventListener("hashchange", (event) => {
            this.setState({ hash: window.location.hash });
        });
    }
    render({ children, no_match }, { hash }) {
        if (!Array.isArray(children)) { children = [ children ]; }  // preact children is just an object if only one child given
        // const hash = window.location.hash;
        const query = new URLSearchParams(window.location.search);
        const el = children.find(x => `#${x.props.path}` == hash || x.props.path == hash);
        return html`${el ? el : no_match()}`;
    }
}