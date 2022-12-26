import init, { draw, spawn_firework, resize_canvas } from "./pkg/new_years.js";

let name = "";
function askForNameThenJoin(ws) {
  const popup = document.getElementById("wyn");
  popup.classList.toggle("close");
  document.getElementById("wyn-button").onclick = () => {
    name = document.getElementById("wyn-input").value;
    if (name.trim() === "") return;
    localStorage.setItem("name", name);
    ws.send(JSON.stringify({ type: "join", name, new: true }));
    popup.classList.toggle("close");
  };
}

async function run() {
  await init();

  const ws = new WebSocket("wss://happy-new-year.deno.dev");

  ws.onopen = () => {
    ws.onmessage = (ev) => {
      const message = JSON.parse(ev.data);
      switch (message.type) {
        case "connected":
          document.getElementById("wyn-message").innerText =
            "Introduce yourself to the world";
          name = localStorage.getItem("name") ?? "";
          if (!name) {
            askForNameThenJoin(ws);
            break;
          }
          ws.send(JSON.stringify({ type: "join", name }));
          break;
        case "error":
          if (message.error === "name_already_exist") {
            document.getElementById("wyn-message").innerText =
              "That username already exist";
            askForNameThenJoin(ws);
          }
          break;
        case "launch":
          spawn_firework(message.name);
          break;
        case "join":
          spawn_firework(message.name);
      }
    };
  };

  setInterval(draw, 10);

  /* Change the canvas resolution when the window is resized. */
  window.onresize = (_event) => {
    resize_canvas();
  };
}

run();
