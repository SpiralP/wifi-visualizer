import vis from "vis";
const oui: (mac: string) => string | null = require("oui");

const iconNameToCode = {
  broadcast_tower: "\uf519",
  circle: "\uf111",
  android: "\uf17b",
  apple: "\uf179",
  amazon: "\uf270",
  desktop: "\uf108",
  mobile: "\uf10b",
};

// tslint:disable-next-line:object-literal-key-quotes
const ouiToIconCode = {
  "Cisco Systems Inc.": iconNameToCode.broadcast_tower,
  "Intel Corporation": iconNameToCode.desktop,
  "LG Electronics (Mobile Communications)": iconNameToCode.android,
  Apple: iconNameToCode.apple,
  "Murata Manufacturing Co. Ltd": iconNameToCode.mobile,
  Broadcom: iconNameToCode.amazon,
  "Samsung Electro-Mechanics(Thailand)": iconNameToCode.mobile,
};

function companyToIconCode(company: string | null) {
  if (company) {
    const iconCode = ouiToIconCode[company];
    if (iconCode) {
      return iconCode;
    } else {
      console.warn(`no icon for ${company}`);
    }
  }
  return iconNameToCode.circle;
}

async function connect(hello: string, callback: (frame: Frame) => void) {
  const ws = new WebSocket("ws://localhost:3012/");

  ws.onmessage = function message(data) {
    callback(JSON.parse(data.data));
  };

  await new Promise((resolve, reject) => {
    ws.onerror = function error(err) {
      reject(err);
    };
    ws.onopen = function open() {
      resolve();
    };
  });

  ws.send(hello);

  await new Promise((resolve, reject) => {
    ws.onerror = function error(err) {
      reject(err);
    };
    ws.onclose = function close() {
      resolve();
    };
  });
}

const nodes = new vis.DataSet();
const edges = new vis.DataSet();

const network = new vis.Network(
  document.getElementById("root")!,
  { nodes, edges },
  {
    groups: {
      access_point: {
        size: 30,
      },
      station: {
        size: 20,
      },
    },
    nodes: {
      shape: "icon",
      icon: {
        face: '"Font Awesome 5 Free", "Font Awesome 5 Brands"',
        code: iconNameToCode.circle,
      },
      // shadow: true,
    },
    edges: {
      width: 2,
      // shadow: true,
    },
  }
);

function isBroadcast(mac: string): boolean {
  return (
    mac === "FF:FF:FF:FF:FF:FF" ||
    mac.startsWith("01:00:5E") || // multicast
    false
  );
}

function hash(mac1: string, mac2: string): string {
  if (mac1 >= mac2) return mac1 + mac2;
  else return mac2 + mac1;
}

declare global {
  interface Window {
    node_cache: {};
    edge_cache: {};
  }
}

const node_cache = {};
const edge_cache = {};
window.node_cache = node_cache;
window.edge_cache = edge_cache;
let frames = 0;

function handleFrame(frame: BasicFrame) {
  // console.log(frame);

  // transmitter -> receiver
  const { transmitter_address, receiver_address } = frame;
  // const subtype = frame.frame_control.type_.Management!;

  if (receiver_address && isBroadcast(receiver_address)) return;

  // const transmitter_node = nodes.get(transmitter_address);
  if (transmitter_address && !node_cache[transmitter_address]) {
    const company = oui(transmitter_address);
    // if (!company) console.warn(`no oui for ${transmitter_address}`);

    nodes.add({
      id: transmitter_address,
      // label: transmitter_address,
      group: "access_point",
      icon: { code: companyToIconCode(company) },
      hover: true,
      title: company || transmitter_address,
    });
    node_cache[transmitter_address] = true;
  }

  // const receiver_node = nodes.get(receiver_address);
  if (receiver_address && !node_cache[receiver_address]) {
    const company = oui(receiver_address);
    // if (!company) console.warn(`no oui for ${receiver_address}`);

    nodes.add({
      id: receiver_address,
      // label: receiver_address,
      group: "station",
      icon: { code: companyToIconCode(company) },
      hover: true,
      title: company || receiver_address,
    });
    node_cache[receiver_address] = true;
  }

  if (transmitter_address && receiver_address) {
    const id = hash(transmitter_address, receiver_address);
    if (!edge_cache[id]) {
      edges.add({
        id,
        from: transmitter_address,
        to: receiver_address,
        frames: 1,
      });
      edge_cache[id] = true;
    } else {
      // it already exists
      const old = edges.get(id) as { frames: number };
      const my_frames = old.frames + 1;
      // edges.update({ id, frames: my_frames, width: 2 });
    }
  }
}

connect(
  "file",
  (data) => {
    const frame = data.Beacon || data.Basic!;
    // console.log(frame.transmitter_address, "->", frame.receiver_address);

    frames += 1;
    handleFrame(frame);
  }
).then(() => {
  console.log(`${frames} frames`);
});

// const graphics = Viva.Graph.View.svgGraphics();
// graphics
//   .node((node) => {
//     return Viva.Graph.svg("circle")
//       .attr("r", node.data.size)
//       .attr("fill", node.data.color);
//   })
//   .placeNode((nodeUI, pos) => {
//     nodeUI.attr("transform", `translate(${pos.x} ${pos.y})`);
//   });

// const renderer = Viva.Graph.View.renderer(graph, {
//   container: document.getElementById("root"),
//   graphics,
// });
// renderer.run();
