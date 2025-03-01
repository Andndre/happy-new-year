import { serve } from "https://deno.land/std@0.170.0/http/mod.ts";
import "https://deno.land/x/dotenv@v3.2.0/load.ts";
import { getUsers, login } from "./database.ts";

type Player = {
  active: boolean;
  ws?: WebSocket;
};

const players: Map<string, Player> = new Map<string, Player>();

const users = await getUsers();

users.forEach((user) => {
  players.set(user.name, { active: false, ws: undefined });
});

type Message =
  | { type: "launch"; name: string }
  | { type: "join"; name: string; new: boolean }
  | { type: "connected" }
  | { type: "leave"; name: string }
  | { type: "error"; error: string };

async function onMessage(ws: WebSocket, message: Message) {
  switch (message.type) {
    case "launch":
      players.forEach((player, pName) => {
        if (message.name !== pName && player.active) {
          player.ws!.send(
            JSON.stringify({ type: "launch", name: message.name })
          );
        }
      });
      break;
    case "join":
      if (players.has(message.name) && message.new) {
        ws.send(
          JSON.stringify({
            type: "error",
            error: "name_already_exist",
          } as Message)
        );
        break;
      }
      ws.send(
        JSON.stringify({ type: "launch", name: message.name } as Message)
      );
      if (!players.has(message.name)) {
        await login(message.name);
      }
      players.set(message.name, { active: true, ws });
      players.forEach((player, name) => {
        if (message.name !== name) {
          if (message.new && player.active) {
            player.ws!.send(JSON.stringify(message));
          }
          ws.send(JSON.stringify({ type: "launch", name } as Message));
        }
      });
      break;
  }
}

function reqHandler(req: Request) {
  if (req.headers.get("upgrade") != "websocket") {
    return new Response(null, { status: 501 });
  }
  const { socket: ws, response } = Deno.upgradeWebSocket(req);
  let name = "";
  ws.onmessage = (ev: MessageEvent<string>) => {
    try {
      console.log(ev.data);
      const message = JSON.parse(ev.data) as Message;
      if (message.type === "join") {
        name = message.name;
      }
      onMessage(ws, message);
    } catch (e) {
      console.error(e);
    }
  };
  ws.onopen = () => {
    ws.send(JSON.stringify({ type: "connected" } as Message));
  };
  ws.onclose = () => {
    if (!players.has(name)) return;
    players.get(name)!.active = false;

    players.forEach((player, pName) => {
      if (pName !== name && player.active) {
        player.ws!.send(JSON.stringify({ type: "leave", name } as Message));
      }
    });
  };
  return response;
}
serve(reqHandler, { port: 8888 });
