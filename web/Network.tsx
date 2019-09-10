import { iconNameToCode } from "./helpers";
import vis from "vis-network";
import React from "react";
import { IToaster } from "@blueprintjs/core";
import copy from "clipboard-copy";

if (!copy) {
  throw new Error("WHAT");
}

interface NetworkProps {
  nodes: { [id: string]: vis.Node };
  edges: { [id: string]: vis.Edge };
  toaster: IToaster;
}

interface NetworkState {}

export class Network extends React.PureComponent<NetworkProps, NetworkState> {
  network?: vis.Network;
  nodes: vis.DataSet<vis.Node> = new vis.DataSet();
  edges: vis.DataSet<vis.Edge> = new vis.DataSet();

  containerRef: React.RefObject<HTMLDivElement> = React.createRef();

  componentDidMount() {
    const { containerRef, edges, nodes } = this;

    if (!containerRef.current) {
      throw new Error("ref not set?");
    }

    this.network = new vis.Network(
      containerRef.current,
      { nodes, edges },
      {
        interaction: {
          hover: true,
        },
        nodes: {
          shape: "icon",
          icon: {
            face: '"Font Awesome 5 Free", "Font Awesome 5 Brands"',
            code: iconNameToCode.circle,
          },
          // shadow: true,
          // shapeProperties: {
          //   interpolation: false, // 'true' for intensive zooming
          // },
        },
        edges: {
          width: 4,
          // color: {color: "#1E7AE5", }
          // shadow: true,
        },
        layout: { improvedLayout: false },
      }
    );

    this.network.moveTo({ scale: 0.75 });

    this.network.on(
      "click",
      (event: { nodes: Array<string>; edges: Array<string> }) => {
        if (event.nodes.length === 1) {
          const addr = event.nodes[0];
          copy(addr)
            .then(() => {
              this.props.toaster.show({ message: `copied "${addr}"` });
            })
            .catch(() => {
              this.props.toaster.show({
                message: `failed to copy`,
                intent: "danger",
              });
            });
        }
      }
    );

    // this.nodes.add({ id: "a" });
    // this.nodes.add({ id: "b" });

    // this.edges.add({ from: "a", to: "b", id: "ab" });

    // @ts-ignore
    window.edges = this.edges;
    // @ts-ignore
    window.nodes = this.nodes;
    // @ts-ignore
    window.vis = vis;

    Object.keys(this.props.nodes).forEach((key) => {
      this.nodes.update(this.props.nodes[key]);
    });

    Object.keys(this.props.edges).forEach((key) => {
      this.edges.update(this.props.edges[key]);
    });
  } // componentDidMount

  componentWillUnmount() {
    if (this.network) {
      this.network.destroy();
    }
  }

  componentDidUpdate(lastProps: NetworkProps) {
    if (this.props.nodes !== lastProps.nodes) {
      Object.keys(this.props.nodes)
        .filter((key) => this.props.nodes[key] !== lastProps.nodes[key])
        .forEach((key) => {
          this.nodes.update(this.props.nodes[key]);
        });
    }

    if (this.props.edges !== lastProps.edges) {
      Object.keys(this.props.edges)
        .filter((key) => this.props.edges[key] !== lastProps.edges[key])
        .forEach((key) => {
          this.edges.update(this.props.edges[key]);
        });
    }
  }

  render() {
    return (
      <div
        style={{ height: "100vh", width: "100vw" }}
        ref={this.containerRef}
      />
    );
  }
}
