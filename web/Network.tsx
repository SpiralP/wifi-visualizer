import { iconNameToCode } from "./helpers";
import vis from "vis-network";
import React from "react";
import { IToaster } from "@blueprintjs/core";
import copy from "clipboard-copy";

interface NetworkProps {
  nodes: { [id: string]: vis.Node };
  edges: { [id: string]: vis.Edge };
  toaster: IToaster;
}

interface NetworkState {}

export default class Network extends React.PureComponent<
  NetworkProps,
  NetworkState
> {
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

    // @ts-ignore
    window.edges = this.edges;
    // @ts-ignore
    window.nodes = this.nodes;
    // @ts-ignore
    window.vis = vis;

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

  updateNode(id: string, node: vis.Node) {
    // console.log(`Network updateNode ${id}`);
    node.id = id;
    this.nodes.update(node);
  }

  updateEdge(id: string, edge: vis.Edge) {
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
