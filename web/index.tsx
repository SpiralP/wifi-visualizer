import {
  hashMacs,
  iconNameToCode,
  companyToIconCode,
  connect,
  byteArrayToString,
  status,
} from "./helpers";
import vis from "vis";
import ReactDOM from "react-dom";
import React, { useState, useEffect } from "react";
import Websocket from "react-websocket";
import oui from "./oui";
import { Network } from "./Network";

// @ts-ignore
if (module.hot != null) {
  // @ts-ignore
  module.hot.dispose(() => {
    // module is about to be replaced
    document.location.reload(true);
  });
}

status("index.tsx");

function ag(id) {
  const company = oui(id);

  const ag = {
    id,
    icon: { code: companyToIconCode(company), size: 50, color: "#ff00ff" },
    title: company ? `${company}<br />${id}` : id,
  };

  return ag;
}

interface AppState {
  connected: boolean;
  nodes: { [id: string]: vis.Node };
  edges: { [id: string]: vis.Edge };
}

class App extends React.Component<{}, AppState> {
  state: AppState = { connected: false, nodes: {}, edges: {} };

  handleFrameEvent(event: FrameEvent) {
    if (event.type === "NewAddress") {
      const id = event.data;
      const company = oui(id);

      // console.log(id);
      if (id === "98-d6-f7-01-01-00") {
        console.log("GOT ME!!!!");

        this.setState({
          nodes: {
            ...this.state.nodes,
            [id]: {
              id,
              icon: {
                code: companyToIconCode(company),
                size: 50,
                color: "#ff00ff",
              },
              title: company ? `${company}<br />${id}` : id,
            },
          },
        });
      } else {
        this.setState({
          nodes: {
            ...this.state.nodes,
            [id]: {
              id,
              icon: { code: companyToIconCode(company), size: 50 },
              title: company ? `${company}<br />${id}` : id,
            },
          },
        });
      }
    } else if (event.type === "AccessPoint") {
      const [id, info] = event.data;
      const { ssid, channel } = info;
      const label = byteArrayToString(ssid);

      const node = this.state.nodes[id];
      const lastTitle = node ? node.title : "";

      this.setState({
        nodes: {
          ...this.state.nodes,
          [id]: {
            id,
            icon: { color: "green" },
            label,
            title: `${lastTitle}<br />channel ${channel}`,
          },
        },
      });
    } else if (event.type === "Connection") {
      const [from, to, kind] = event.data;
      const id = hashMacs(from, to);

      if (kind === "Associated") {
        this.setState({
          edges: {
            ...this.state.edges,
            [id]: {
              id,
              from,
              to,
              color: { color: "blue", highlight: "blue", hover: "blue" },
              dashes: false,
              width: 3,
            },
          },
        });
      } else if (kind === "Authentication") {
        this.setState({
          edges: {
            ...this.state.edges,
            [id]: {
              id,
              from,
              to,
              color: { color: "green", highlight: "green", hover: "green" },
              dashes: false,
              width: 3,
            },
          },
        });
      } else if (kind === "Disassociated") {
        this.setState({
          edges: {
            ...this.state.edges,
            [id]: {
              id,
              from,
              to,
              color: { color: "red", highlight: "red", hover: "red" },
              dashes: true,
              width: 3,
            },
          },
        });
      } else if (kind === "InRange") {
        this.setState({
          edges: {
            ...this.state.edges,
            [id]: {
              id,
              from,
              to,
              color: { color: "grey", highlight: "grey", hover: "grey" },
              dashes: true,
              width: 0.1,
            },
          },
        });
        // network.clusterByConnection(from);
      }
    } else if (event.type === "ProbeRequest") {
      const [from, ssidBytes] = event.data;
      const ssid = byteArrayToString(ssidBytes);

      const node = { ...this.state.nodes[from], label: ssid };
      this.setState({ nodes: { ...this.state.nodes, [from]: node } });
    } else if (event.type === "InactiveAddress") {
      const addrs = event.data;
      addrs.forEach((id) => {
        // TODO don't edit this.state directly!
        this.state.nodes[id] = { ...this.state.nodes[id], icon: { size: 25 } };
      });
      this.setState({ nodes: { ...this.state.nodes } });
    } else {
      console.warn(event);
    }
  }

  handleMessage(msg: string) {
    const event: FrameEvent = JSON.parse(msg);
    this.handleFrameEvent(event);
  }

  render() {
    const { nodes, edges } = this.state;

    return (
      <div>
        <Network nodes={nodes} edges={edges} />
        <Websocket
          url="ws://127.0.0.1:8001/"
          onMessage={(msg: string) => this.handleMessage(msg)}
          onOpen={() => {
            console.log("opened");
            this.setState({ connected: true });
          }}
          onClose={() => {
            console.log("closed");
            this.setState({ connected: false });
          }}
          debug={true}
          reconnect={false}
        />
      </div>
    );
  }
}

const root = document.getElementById("root");
if (root) {
  ReactDOM.render(<App />, root);
} else {
  console.error("no root element!");
}
