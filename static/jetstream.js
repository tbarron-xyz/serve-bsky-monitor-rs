export class Jetstream {
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