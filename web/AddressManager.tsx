import React from "react";
import Network from "./Network";
import { IToaster } from "@blueprintjs/core";
import vis from "vis-network";
import { companyToIconCode, hashMacs, byteArrayToString } from "./helpers";
import oui from "./oui";

export interface AddressOptions {
  connections?: { [id: string]: ConnectionType };

  accessPointInfo?: {
    ssidBytes: Array<number>;
    channel?: number;
  };

  probeRequests?: Array<string>;

  loss?: number;
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
    address: AddressOptions,
    nodes: { [id: string]: vis.Node },
    edges: { [id: string]: vis.Edge }
  ) {
    console.log(`AddressManager updateNetwork ${id}`);

    // } else if (kind === "Authentication") {

    //         color: { color: "green", highlight: "green", hover: "green" },
    //         dashes: false,
    //         width: 3,
    //       },
    //     },
    //   });
    // } else if (kind === "Disassociated") {

    //         color: { color: "red", highlight: "red", hover: "red" },
    //         dashes: true,
    //         width: 3,
    //       },
    //     },
    //   });
    // } else if (kind === "InRange") {

    //         color: { color: "grey", highlight: "grey", hover: "grey" },
    //         dashes: true,
    //         width: 0.1,
    //       },
    //     },
    //   });
    //   // network.clusterByConnection(from);
    // }
    // probe
    // const id = hashMacs(from, to);
    // loss
    // const interpolateColor = interpolateLab("#2B7CE9", "#FF0000");
    // const color = interpolateColor(percentLoss);

    const { accessPointInfo, loss } = address;

    const known = ["98-d6-f7-01-01-00", "48-a4-72-1b-d3-43"];

    const color =
      known.indexOf(id) !== -1
        ? "#ff00ff"
        : accessPointInfo
        ? "green"
        : "#2B7CE9";

    const company = oui(id);

    let title = company ? `${company}<br />${id}` : id;
    let label = loss ? `${Math.round(loss * 100)}% loss` : "";

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
        size: 50,
        color,
      },
    };

    if (address.connections) {
      Object.entries(address.connections).forEach(([otherId, kind]) => {
        const edgeId = hashMacs(id, otherId);

        const color =
          kind === "Associated"
            ? "blue"
            : kind === "Authentication"
            ? "green"
            : kind === "Disassociated"
            ? "red"
            : kind === "InRange"
            ? "grey"
            : "black";

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

  updateAddresses(addresses: { [id: string]: AddressOptions }) {
    const { nodes: a, edges: b } = this.state;
    const nodes = { ...a };
    const edges = { ...b };

    Object.entries(addresses).forEach(([id, address]) => {
      this.updateNetwork(id, address, nodes, edges);
    });

    this.setState({ nodes, edges });
  }

  componentDidMount() {
    this.updateAddresses(this.props.addresses);
  }

  componentWillReceiveProps(nextProps: AddressManagerProps) {
    if (nextProps.addresses !== this.props.addresses) {
      const o = {};
      Object.entries(nextProps.addresses).forEach(([id, address]) => {
        if (address !== this.props.addresses[id]) {
          o[id] = address;
        }
      });
      this.updateAddresses(o);
    }
  }

  render() {
    const { toaster } = this.props;
    const { nodes, edges } = this.state;
    console.log("AddressManager render", nodes, edges);

    return <Network nodes={nodes} edges={edges} toaster={toaster} />;
  }
}
