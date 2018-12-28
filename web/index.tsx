import {
  hashMacs,
  setNamedTimeout,
  iconNameToCode,
  companyToIconCode,
  connect,
  byteArrayToString,
  htmlEscape,
} from "./helpers";
import copy from "clipboard-copy";
import oui from "./oui";
import vis from "vis";

const nodes: vis.DataSet<vis.Node> = new vis.DataSet();
const edges: vis.DataSet<vis.Edge> = new vis.DataSet();

// @ts-ignore
window.nodes = nodes;
// @ts-ignore
window.edges = edges;

const network = new vis.Network(
  document.getElementById("root")!,
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

// @ts-ignore
window.network = network;

network.moveTo({ scale: 0.4 });

network.on("click", (event: { nodes: Array<string>; edges: Array<string> }) => {
  if (event.nodes.length === 1) {
    console.log(event);
    copy(event.nodes[0])
      .then(() => console.log("copied"))
      .catch(() => console.warn("failed to copy"));
  }
});

// network.cluster({
//   joinCondition: (nodeOptions) => {
//     console.log(nodeOptions);
//     return nodeOptions.cid === 1;
//   },
// });

function handleFrameEvent(event: FrameEvent) {
  if (event.type === "NewAddress") {
    const mac = event.data;
    const company = oui(mac);

    nodes.add({
      id: mac,
      icon: { code: companyToIconCode(company) },
      title: company ? `${htmlEscape(company)}<br />${mac}` : mac,
    });
  } else if (event.type === "SetKind") {
    const [id, kind] = event.data;

    if (kind.type === "AccessPoint") {
      const ssid = kind.data;
      const label = byteArrayToString(ssid);

      nodes.update({ id, icon: { color: "green" }, label });
    } else if (kind.type === "Station") {
      // nodes.update({ id, icon: { color: "blue" } });
    }
  } else if (event.type === "Connection") {
    const [from, to, kind] = event.data;
    const id = hashMacs(from, to);

    if (kind === "Associated") {
      edges.remove(id);
      edges.add({
        id,
        from,
        to,
      });
    } else if (kind === "Disassociated") {
      edges.remove(id);
      edges.add({
        id,
        from,
        to,
        dashes: true,
        color: { color: "red", highlight: "red", hover: "red" },
      });
    } else if (kind === "InRange") {
      edges.remove(id);
      edges.add({
        id,
        from,
        to,
        dashes: true,
        width: 0.1,
      });
      // network.clusterByConnection(from);
    }
  } else if (event.type === "ProbeRequest") {
    const [from, ssidBytes] = event.data;
    const ssid = byteArrayToString(ssidBytes);
    nodes.update({ id: from, title: ssid });
  } else {
    console.warn(event);
  }
}

let firstFrame: number;
connect(
  "file",
  (data) => {
    if (!firstFrame) {
      firstFrame = Date.now();
    }

    handleFrameEvent(data);
    setNamedTimeout(
      "bap",
      () => {
        console.log(`burst took ${Date.now() - 1000 - firstFrame} ms`);
      },
      1000
    );
  }
)
  .then(() => {})
  .catch((e) => console.error(e));
