import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import "vis-network/dist/vis-network.css"; // for popups
import "@fortawesome/fontawesome-free/css/solid.css";
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import React from "react";
import { storiesOf } from "@storybook/react";
import Network from "../web/Network";
import { Toaster } from "@blueprintjs/core";
import { companyToIconCode, ouiToIconCode } from "../web/helpers";
import { interpolateLab } from "d3-interpolate";

const toaster = Toaster.create({ position: "top-right" });

const randomColor = (() => {
  "use strict";

  const randomInt = (min: number, max: number) => {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  };

  return () => {
    var h = randomInt(0, 360);
    var s = randomInt(42, 98);
    var l = randomInt(40, 90);
    return `hsl(${h},${s}%,${l}%)`;
  };
})();

function makeid(length: number) {
  var result = "";
  var characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  var charactersLength = characters.length;
  for (var i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

function makeNode() {
  const id = makeid(2);

  const companies = Object.keys(ouiToIconCode);
  const company = companies[Math.floor(Math.random() * companies.length)];
  const code = companyToIconCode(company);

  const color = randomColor();

  return {
    id: id,
    icon: {
      code,
      size: 50,
      color,
    },
    title: `${id}'s title!`,
    label: `${id}'s label!`,
  };
}

function makeNodes(n: number) {
  const o = {};
  for (let index = 0; index < n; index++) {
    const node = makeNode();
    o[node.id] = node;
  }
  return o;
}

function makeEdgeName(a: string, b: string) {
  if (a > b) {
    return a + b;
  } else {
    return b + a;
  }
}
function makeEdge(nodes: string[]) {
  const from = nodes[Math.floor(Math.random() * nodes.length)];
  const to = nodes[Math.floor(Math.random() * nodes.length)];

  const id = makeEdgeName(from, to);

  return {
    id,
    from,
    to,
  };
}

function makeEdges(n: number, nodes: string[]) {
  const o = {};
  for (let index = 0; index < n; index++) {
    const edge = makeEdge(nodes);
    o[edge.id] = edge;
  }
  return o;
}

storiesOf("Network", module)
  .addParameters({ options: { showPanel: false } })
  .add("Nodes", () => (
    <Network nodes={makeNodes(10)} edges={{}} toaster={toaster} />
  ))
  .add("Nodes with Edges", () => {
    const nodes = makeNodes(10);
    const edges = makeEdges(10, Object.keys(nodes));

    return <Network nodes={nodes} edges={edges} toaster={toaster} />;
  })
  .add("Test loss", () => {
    return <LossTester />;
  });

class LossTester extends React.PureComponent<
  {},
  { nodes: { [id: string]: vis.Node }; edges: { [id: string]: vis.Edge } }
> {
  constructor(props) {
    super(props);

    const nodes: { [id: string]: vis.Node } = {
      A: {
        id: "A",
        title: "A",
        label: "A",
      },
      B: {
        id: "B",
        title: "B",
        label: "B",
      },
    };
    const edges: { [id: string]: vis.Edge } = {
      AB: {
        id: "AB",
        from: "A",
        to: "B",
        color: "#2B7CE9",
        physics: false,
        width: 10,
      },
    };

    this.state = { nodes, edges };

    const a = (newObj: vis.Edge) => {
      const AB = { ...this.state.edges.AB, ...newObj };

      this.setState({ edges: { AB } });
    };

    const t = (n: number) =>
      new Promise((a) => {
        setTimeout(a, n);
      });

    const interpolateColor = interpolateLab("#2B7CE9", "#FF0000");
    const update = (ratePerSecond: number, percentLoss: number) => {
      const color = interpolateColor(percentLoss);
      a({
        label: `${Math.floor(ratePerSecond)} pps\n${Math.floor(
          percentLoss * 100
        )}% loss`,
        width: ratePerSecond,
        color,
        length: 1000,
        dashes: [10, 20 * (percentLoss * 2)],
      });
    };

    (async () => {
      update(0, 0);
      await t(1000);
      update(10, 0);
      await t(1000);
      update(20, 0);

      for (let i = 0; i < 100; i++) {
        await t(100);
        update(20 - 10 * (i / 100), i / 100);
      }
    })();
  }

  render() {
    const { nodes, edges } = this.state;

    return <Network nodes={nodes} edges={edges} toaster={toaster} />;
  }
}
