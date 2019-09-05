network.on("click", (event: { nodes: Array<string>; edges: Array<string> }) => {
  if (event.nodes.length === 1) {
    const addr = event.nodes[0];
    copy(addr)
      .then(() => console.log(`copied ${addr}`))
      .catch(() => console.warn("failed to copy"));
  }
});
