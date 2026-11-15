import { DurableObject } from "cloudflare:workers";
import LeptosWorker from "./index.js";
import { withDispatch } from "./dispatch.mjs";
const SharedWorker = withDispatch(LeptosWorker);

const DEFAULT_ROOM = "lobby";
const MAX_ROOM_NAME_LENGTH = 64;
const MAX_MESSAGE_LENGTH = 1_000;

function isWebSocketUpgrade(request) {
  return request.headers.get("Upgrade")?.toLowerCase() === "websocket";
}

function normalizeRoomName(value) {
  return (value || DEFAULT_ROOM)
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9:_-]/g, "-")
    .replace(/-+/g, "-")
    .slice(0, MAX_ROOM_NAME_LENGTH)
    || DEFAULT_ROOM;
}

function realtimeRoomName(request) {
  const url = new URL(request.url);
  return `room:${normalizeRoomName(url.searchParams.get("room"))}`;
}

function authorizeRealtimeRequest(_request) {
  // Replace this placeholder with session/auth checks before production use.
  return true;
}

function websocketUpgradeRequired() {
  return new Response("WebSocket upgrade required.", {
    status: 426,
    headers: { Upgrade: "websocket" },
  });
}

async function handleRealtimeSocket(request, env) {
  if (!isWebSocketUpgrade(request)) {
    return websocketUpgradeRequired();
  }

  if (!authorizeRealtimeRequest(request)) {
    return new Response("Unauthorized realtime connection.", { status: 401 });
  }

  const room = env.REALTIME_ROOM.getByName(realtimeRoomName(request));
  return room.fetch(request);
}

export class RealtimeRoom extends DurableObject {
  constructor(ctx, env) {
    super(ctx, env);

    ctx.blockConcurrencyWhile(async () => {
      this.ctx.storage.sql.exec(`
        CREATE TABLE IF NOT EXISTS messages (
          id TEXT PRIMARY KEY,
          text TEXT NOT NULL,
          sent_at TEXT NOT NULL
        );
      `);
    });
  }

  async fetch(request) {
    if (!isWebSocketUpgrade(request)) {
      return websocketUpgradeRequired();
    }

    const pair = new WebSocketPair();
    const [client, server] = Object.values(pair);
    const room = realtimeRoomName(request);
    const connectedAt = new Date().toISOString();

    this.ctx.acceptWebSocket(server, [room]);
    server.serializeAttachment({ room, connectedAt });
    server.send(JSON.stringify({
      type: "ready",
      room,
      connectedAt,
      connections: this.ctx.getWebSockets(room).length,
    }));

    return new Response(null, {
      status: 101,
      webSocket: client,
    });
  }

  async webSocketMessage(ws, message) {
    if (typeof message !== "string") {
      ws.send(JSON.stringify({ type: "error", error: "Only text messages are supported." }));
      return;
    }

    const text = message.trim();
    if (!text || text.length > MAX_MESSAGE_LENGTH) {
      ws.send(JSON.stringify({
        type: "error",
        error: `Message must be 1-${MAX_MESSAGE_LENGTH} characters.`,
      }));
      return;
    }

    const attachment = ws.deserializeAttachment() ?? { room: DEFAULT_ROOM };
    const room = attachment.room ?? DEFAULT_ROOM;
    const payload = {
      type: "message",
      id: crypto.randomUUID(),
      room,
      text,
      sentAt: new Date().toISOString(),
      connections: this.ctx.getWebSockets(room).length,
    };

    this.ctx.storage.sql.exec(
      "INSERT INTO messages (id, text, sent_at) VALUES (?, ?, ?)",
      payload.id,
      payload.text,
      payload.sentAt,
    );

    this.broadcast(room, payload);
  }

  async webSocketClose(_ws, _code, _reason, _wasClean) {
    // With this repo's compatibility date, Cloudflare auto-replies to close frames.
  }

  broadcast(room, payload) {
    const encoded = JSON.stringify(payload);

    for (const socket of this.ctx.getWebSockets(room)) {
      if (socket.readyState === WebSocket.OPEN) {
        socket.send(encoded);
      }
    }
  }
}

export default class extends SharedWorker {
  handleRealtimeSocket(request) {
    return handleRealtimeSocket(request, this.env);
  }
}
