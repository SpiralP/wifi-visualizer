const WebSocket = require("ws");

const ws = new WebSocket("ws://localhost:3012/");

ws.on("open", function open() {
  ws.send("test");
});

ws.on("message", function incoming(data) {
  console.log(data);
});
