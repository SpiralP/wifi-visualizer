import React from "react";
import Network from "./Network";
import { IToaster } from "@blueprintjs/core";
import vis from "vis-network";
import { companyToIconCode, hashMacs, byteArrayToString } from "./helpers";
import { oui } from "./oui";

const known = ["98-d6-f7-01-01-00", "48-a4-72-1b-d3-43"];

const connectionTypeToColor: { [kind: string]: string } = {
  Associated: "blue",
  Authentication: "green",
  Disassociated: "red",
  InRange: "grey",
};

export interface AddressOptions {
  connections?: { [id: string]: ConnectionType };

  accessPointInfo?: {
    ssidBytes: Array<number>;
    channel?: number;
  };

  probeRequests?: Array<string>;

  // loss?: number;

  signal?: number;
}

interface AddressManagerProps {
  addresses: { [id: string]: AddressOptions };
  toaster: IToaster;
}

interface AddressManagerState {
  nodes: { [id: string]: vis.Node };
  edges: { [id: string]: vis.Edge };
}

export default class AddressManager extends React.PureComponent<
  AddressManagerProps,
  AddressManagerState
> {
  state: AddressManagerState = {
    nodes: {},
    edges: {},
  };

  updateNetwork(
    id: MacAddress,
    lastAddress: AddressOptions,
    address: AddressOptions,
    nodes: { [id: string]: vis.Node },
    edges: { [id: string]: vis.Edge }
  ) {
    // console.log(`AddressManager updateNetwork ${id}`);

    const { accessPointInfo, signal } = address;

    const color =
      known.indexOf(id) !== -1
        ? "#ff00ff"
        : accessPointInfo
        ? "green"
        : "#2B7CE9";

    const company = oui(id);

    let title = company ? `${company}<br />${id}` : id;
    let label = signal ? `${signal} dBm` : "";

    const size = signal ? Math.max(100 + signal, 5) : 30;

    if (accessPointInfo) {
      const { ssidBytes, channel } = accessPointInfo;
      const ssid = byteArrayToString(ssidBytes);

      label = ssid;
      title += `<br />channel ${channel}`;
    }

    nodes[id] = {
      id,
      title,
      label,
      icon: {
        code: companyToIconCode(company),
        size,
        color,
      },
    };

    if (
      address.connections &&
      address.connections !== lastAddress.connections
    ) {
      const lastConnections = lastAddress.connections;
      Object.entries(address.connections).forEach(([otherId, kind]) => {
        if (lastConnections && kind === lastConnections[otherId]) {
          return;
        }

        const edgeId = hashMacs(id, otherId);
        const color = connectionTypeToColor[kind];
        const dashes = kind === "Disassociated" || kind === "InRange";
        const width = kind === "InRange" ? 0.1 : 3;

        edges[edgeId] = {
          id: edgeId,
          from: id,
          to: otherId,
          color: { color: color, highlight: color, hover: color },
          dashes,
          width,
        };
      });
    }
  }

  updateAddresses(
    lastAddresses: { [id: string]: AddressOptions },
    addresses: { [id: string]: AddressOptions }
  ) {
    const { nodes: a, edges: b } = this.state;
    const nodes = { ...a };
    const edges = { ...b };

    Object.entries(addresses).forEach(([id, address]) => {
      const lastAddress = lastAddresses[id] || {};
      this.updateNetwork(id, lastAddress, address, nodes, edges);
    });

    this.setState({ nodes, edges });
  }

  componentDidMount() {
    this.updateAddresses({}, this.props.addresses);
  }

  componentWillReceiveProps(nextProps: AddressManagerProps) {
    if (nextProps.addresses !== this.props.addresses) {
      const o = {};
      Object.entries(nextProps.addresses).forEach(([id, address]) => {
        if (address !== this.props.addresses[id]) {
          o[id] = address;
        }
      });
      this.updateAddresses(this.props.addresses, o);
    }
  }

  render() {
    const { toaster } = this.props;
    const { nodes, edges } = this.state;
    // console.log("AddressManager render", nodes, edges);

    return <Network nodes={nodes} edges={edges} toaster={toaster} />;
  }
}
