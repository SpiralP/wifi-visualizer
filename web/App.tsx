import * as React from "react";
import { RandomizeNodePositions, RelativeSize, Sigma } from "react-sigma";

async function connect(callback: (frame: Frame) => void) {
  const ws = new WebSocket("ws://localhost:3012/");

  ws.onmessage = function message(data) {
    callback(JSON.parse(data.data));
  };

  await new Promise((resolve, reject) => {
    ws.onerror = function error(err) {
      reject(err);
    };
    ws.onopen = function open() {
      resolve();
    };
  });

  ws.send("test");
}

export default class App extends React.Component {
  state: { nodes: SigmaNode[]; edges: SigmaEdge[] } = {
    nodes: [
      { id: "a", label: "A" },
      { id: "b", label: "B" },
      { id: "c", label: "C" },
    ],
    edges: [
      { id: "a_to_b", source: "a", target: "b", label: "A -> B" },
      { id: "b_to_c", source: "b", target: "c", label: "B -> C" },
      { id: "c_to_a", source: "c", target: "a", label: "C -> A" },
      { id: "b_to_a", source: "b", target: "a", label: "B -> A" },
    ],
  };

  handleFrame(frame: BasicFrame) {
    const nodes = this.state.nodes;
    let id = frame.transmitter_address;
    if (!nodes.find((node) => node.id === id)) {
      nodes.push({ id, label: id });
    }

    id = frame.receiver_address;
    if (!nodes.find((node) => node.id === id)) {
      nodes.push({ id, label: id });
    }

    const edges = this.state.edges;
    edges.push({
      id: frame.transmitter_address + frame.receiver_address,
      source: frame.transmitter_address,
      target: frame.receiver_address,
      label: frame.transmitter_address + frame.receiver_address,
    });

    this.setState({ edges: Array.from(edges), nodes: Array.from(nodes) });
  }

  componentDidMount() {
    connect((data) => {
      const frame = data.Beacon || data.Basic!;
      console.log(frame.transmitter_address, "->", frame.receiver_address);

      this.handleFrame(frame);
    });
  }

  render() {
    const { edges, nodes } = this.state;
    console.log(edges, nodes);

    return (
      <Sigma
        renderer="canvas"
        graph={{ edges, nodes }}
        settings={{ drawEdges: true, drawEdgeLabels: true, clone: false }}
        style={{ flex: 1, width: "100%", height: "100%" }}
      >
        <RandomizeNodePositions>
          <RelativeSize initialSize={15} />
        </RandomizeNodePositions>
      </Sigma>
    );
  }
}
