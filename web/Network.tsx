import { IToaster } from "@blueprintjs/core";
import copy from "clipboard-copy";
import React from "react";
import { DataSet } from "vis-data";
import { Edge, Network, Node } from "vis-network";
import { iconNameToCode } from "./helpers";

interface NetworkProps {
  nodes: { [id: string]: Node };
  edges: { [id: string]: Edge };
  toaster: IToaster;
}

interface NetworkState {}

export default class NetworkElement extends React.PureComponent<
  NetworkProps,
  NetworkState
> {
  network?: Network;
  nodes: DataSet<Node> = new DataSet();
  edges: DataSet<Edge> = new DataSet();

  containerRef: React.RefObject<HTMLDivElement> = React.createRef();

  componentDidMount() {
    const { containerRef, edges, nodes } = this;

    if (!containerRef.current) {
      throw new Error("ref not set?");
    }

    this.network = new Network(
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

    // @ts-ignore
    window.edges = this.edges;
    // @ts-ignore
    window.nodes = this.nodes;

    Object.entries(this.props.nodes).forEach(([id, node]) => {
      this.updateNode(id, node);
    });

    Object.entries(this.props.edges).forEach(([id, edge]) => {
      this.updateEdge(id, edge);
    });
  } // componentDidMount

  componentWillUnmount() {
    if (this.network) {
      this.network.destroy();
    }
  }

  componentWillReceiveProps(nextProps: NetworkProps) {
    // console.log("Network componentWillReceiveProps", nextProps);

    if (nextProps.nodes !== this.props.nodes) {
      Object.entries(nextProps.nodes).forEach(([id, node]) => {
        if (node !== this.props.nodes[id]) {
          this.updateNode(id, node);
        }
      });
    }

    if (nextProps.edges !== this.props.edges) {
      Object.entries(nextProps.edges).forEach(([id, edge]) => {
        if (edge !== this.props.edges[id]) {
          this.updateEdge(id, edge);
        }
      });
    }
  }

  updateNode(id: string, node: Node) {
    // console.log(`Network updateNode ${id}`);
    node.id = id;
    this.nodes.update(node);
  }

  updateEdge(id: string, edge: Edge) {
    // console.log(`Network updateEdge ${id}`);

    edge.id = id;
    this.edges.update(edge);
  }

  // shouldComponentUpdate() {
  //   return false;
  // }

  render() {
    // console.log("Network render");
    return (
      <div
        style={{ height: "100vh", width: "100vw" }}
        ref={this.containerRef}
      />
    );
  }
}
