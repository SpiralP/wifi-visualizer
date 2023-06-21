import { IToaster } from "@blueprintjs/core";
import React from "react";
import vis from "vis-network";
import NetworkElement from "./Network";
import { companyToIconCode, hashMacs } from "./helpers";
import { ConnectionType, MacAddress } from "./interfaceTypes";
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
    ssid: string;
    channel?: number;
  };
  beaconQuality?: number;

  probeRequests?: Array<string>;

  // loss?: number;

  signal?: number | false;
  rate?: number | false;

  hovered?: boolean;
}

interface AddressNetworkProps {
  addresses: { [id: string]: AddressOptions };
  toaster: IToaster;
}

interface AddressNetworkState {
  nodes: { [id: string]: vis.Node };
  edges: { [id: string]: vis.Edge };
}

export default class AddressNetwork extends React.PureComponent<
  AddressNetworkProps,
  AddressNetworkState
> {
  state: AddressNetworkState = {
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

    const { accessPointInfo, signal, rate, beaconQuality, hovered } = address;

    const color =
      known.indexOf(id) !== -1
        ? "#ff00ff"
        : accessPointInfo
        ? "green"
        : "#2B7CE9";

    const company = oui(id);

    let label = "";
    let title = company ? `${id} (${company})` : id;

    if (accessPointInfo) {
      const { ssid, channel } = accessPointInfo;

      label += ssid;
      title += `<br />channel ${channel}`;
    }

    if (signal) {
      label += `\n${signal} dBm`;
    }

    if (rate) {
      label += `\n${rate} pps`;
    }

    if (beaconQuality) {
      label += `\n${Math.floor(beaconQuality * 100)}% beacons`;
    }

    const size = hovered ? 100 : signal ? Math.max(100 + signal, 5) : 30;

    const code = companyToIconCode(company);

    nodes[id] = {
      id,
      title,
      label,
      icon: {
        code,
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

  componentWillReceiveProps(nextProps: AddressNetworkProps) {
    if (nextProps.addresses !== this.props.addresses) {
      const o: Record<string, AddressOptions> = {};
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
    // display: "flex", flexDirection: "row"
    return <NetworkElement nodes={nodes} edges={edges} toaster={toaster} />;
  }
}
