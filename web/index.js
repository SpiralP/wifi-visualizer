const ws = new WebSocket("ws://localhost:3012/");

ws.addEventListener("open", function open() {
  ws.send("test");
});

ws.addEventListener("message", function incoming(data) {
  console.log(JSON.parse(data.data));
});
