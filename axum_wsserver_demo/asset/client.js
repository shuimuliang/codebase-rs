const ws = new WebSocket('ws://127.0.0.1:3000');
ws.onopen = () => {
  console.log('WebSocket connection established');
}
ws.onmessage = (event) => {
  console.log(`Received message: ${event.data}`);
}
ws.onerror = (error) => {
  console.error(`WebSocket error: ${error}`);
}
ws.onclose = (event) => {
  console.log('WebSocket connection closed');
}