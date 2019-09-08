import { Intent, Alert, IToaster } from "@blueprintjs/core";
import {
  hashMacs,
  companyToIconCode,
  byteArrayToString,
  status,
} from "./helpers";
import vis from "vis";
import React from "react";
import Websocket from "react-websocket";
import oui from "./oui";
import { Network } from "./Network";

interface AppProps {
  toaster: IToaster;
}

interface AppState {
  connected: boolean;
  nodes: { [id: string]: vis.Node };
  edges: { [id: string]: vis.Edge };
  error?: string;
}

export class App extends React.Component<AppProps, AppState> {
  state: AppState = {
    connected: false,
    nodes: {},
    edges: {},
    error: undefined,
  };

  handleFrameEvent(event: FrameEvent) {
    if (event.type === "NewAddress") {
      const id = event.data;
      const company = oui(id);

      // console.log(id);
      if (id === "98-d6-f7-01-01-00" || id == "48-a4-72-1b-d3-43") {
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
      const changed = {};
      addrs.forEach((id) => {
        changed[id] = { ...this.state.nodes[id], icon: { size: 25 } };
      });
      this.setState({ nodes: { ...this.state.nodes, ...changed } });
    } else if (event.type === "Error") {
      const error = event.data;
      console.warn("Error", error);
      this.setState({ error });
    } else {
      console.warn(event);
    }
  }

  handleMessage(msg: string) {
    const event: FrameEvent = JSON.parse(msg);
    this.handleFrameEvent(event);
  }

  render() {
    const { toaster } = this.props;
    const { nodes, edges, error } = this.state;

    return (
      <div>
        <Alert
          isOpen={error ? true : false}
          icon="error"
          intent={Intent.DANGER}
          confirmButtonText="Okay"
          canOutsideClickCancel={true}
          onClose={() => {
            this.setState({ error: undefined });
          }}
        >
          <p>
            Error: <b>{error ? error : "<unknown>"}</b>
          </p>
        </Alert>

        <Network nodes={nodes} edges={edges} toaster={toaster} />
        <Websocket
          url={`ws://${location.host}/ws`}
          onMessage={(msg: string) => this.handleMessage(msg)}
          onOpen={() => {
            status("websocket opened");
            this.setState({ connected: true });
          }}
          onClose={() => {
            status("websocket closed");
            toaster.show({
              message: "websocket closed",
              intent: "danger",
            });
            this.setState({ connected: false });
          }}
          debug={true}
          reconnect={false}
        />
      </div>
    );
  }
}
