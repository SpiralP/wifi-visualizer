import {
  hashMacs,
  iconNameToCode,
  companyToIconCode,
  connect,
  byteArrayToString,
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
    const id = event.data;
    const company = oui(id);

    // console.log(id);
    if (id === "98-d6-f7-01-01-00") {
      console.log("GOT ME!!!!");

      nodes.update({
        id,
        icon: { code: companyToIconCode(company), size: 50, color: "#ff00ff" },
        title: company ? `${company}<br />${id}` : id,
      });
    } else {
      nodes.update({
        id,
        icon: { code: companyToIconCode(company), size: 50 },
        title: company ? `${company}<br />${id}` : id,
      });
    }
  } else if (event.type === "AccessPoint") {
    const [id, info] = event.data;
    const { ssid, channel } = info;
    const label = byteArrayToString(ssid);

    const node = nodes.get(id)!;
    const lastTitle = node ? node.title : "";

    nodes.update({
      id,
      icon: { color: "green" },
      label,
      title: `${lastTitle}<br />channel ${channel}`,
    });
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
    nodes.update({ id: from, label: ssid });
  } else if (event.type === "InactiveAddress") {
    const addrs = event.data;
    addrs.forEach((id) => {
      nodes.update({
        id,
        icon: { size: 25 },
      });
    });
  } else {
    console.warn(event);
  }
}

export default function start(ifname: string = "live/wlan0mon") {
  let firstFrame: number;
  return connect(
    ifname,
    (data) => {
      if (!firstFrame) {
        firstFrame = Date.now();
      }

      handleFrameEvent(data);
      // setNamedTimeout(
      //   "bap",
      //   () => {
      //     console.log(`burst took ${Date.now() - 1000 - firstFrame} ms`);
      //   },
      //   1000
      // );
    }
  )
    .then(() => {
      console.log("ws done");
    })
    .catch((e) => console.warn(e));
}
