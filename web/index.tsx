// TODO import fontawesome for offline

import vis from "vis";
const oui: (mac: string) => string | null = require("oui");
const copy = require("clipboard-copy");
import { hashMacs, isBroadcast, setNamedTimeout } from "./helpers";
const jsesc = require("jsesc");

const iconNameToCode = {
  broadcast_tower: "\uf519",
  circle: "\uf111",
  android: "\uf17b",
  apple: "\uf179",
  amazon: "\uf270",
  desktop: "\uf109",
  mobile: "\uf10b",
  tv: "\uf26c",
  steam: "\uf1b6",
};

// tslint:disable-next-line:object-literal-key-quotes
const ouiToIconCode = {
  "Cisco Systems Inc.": iconNameToCode.broadcast_tower,
  "Belkin International Inc.": iconNameToCode.broadcast_tower,
  "ASUSTek Computer Inc.": iconNameToCode.broadcast_tower,
  "ARRIS Group, Inc.": iconNameToCode.broadcast_tower,
  "TP-Link Technologies Co. Ltd": iconNameToCode.broadcast_tower,
  "Texas Instruments": iconNameToCode.broadcast_tower,
  Netgear: iconNameToCode.broadcast_tower,
  Broadcom: "broadcom",
  "Intel Corporation": iconNameToCode.desktop,
  "LG Electronics (Mobile Communications)": iconNameToCode.android,
  Apple: iconNameToCode.apple,
  "Murata Manufacturing Co. Ltd": iconNameToCode.mobile,
  "Samsung Electro-Mechanics(Thailand)": iconNameToCode.mobile,
  "Roku, Inc": iconNameToCode.tv,
  "Valve Corporation": iconNameToCode.steam,
  "Amazon Technologies Inc.": iconNameToCode.amazon,
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

async function connect(hello: string, callback: (event: FrameEvent) => void) {
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
      width: 2,
      // shadow: true,
    },
    layout: { improvedLayout: false },
  }
);

network.moveTo({ scale: 0.4 });

network.on("click", (event: { nodes: Array<string>; edges: Array<string> }) => {
  if (event.nodes.length === 1) {
    console.log(event);
    copy(event.nodes[0])
      .then(() => console.log("copied"))
      .catch(() => console.warn("failed to copy"));
  }
});

// @ts-ignore
window.network = network;

function htmlEscape(input: string): string {
  // TODO
  return input;
}

function byteArrayToString(input: number[]): string {
  return jsesc(Buffer.from(input).toString());
}

function handleFrameEvent(event: FrameEvent) {
  if (event.type === "NewAddress") {
    const id = event.data;
    const company = oui(id);

    nodes.add({
      id,
      icon: { code: companyToIconCode(company) },
      hover: true,
      title: company ? `${htmlEscape(company)}<br />${id}` : id,
    });
  } else if (event.type === "Connection") {
    const from = event.data[0];
    const to = event.data[1];

    edges.add({
      id: hashMacs(from, to),
      from,
      to,
    });
  } else if (event.type === "SetKind") {
    const id = event.data[0];
    const kind = event.data[1];

    if (kind.type === "AccessPoint") {
      const label = byteArrayToString(kind.data);

      nodes.update({ id, icon: { color: "green" }, label });
    } else if (kind.type === "Station") {
      // nodes.update({ id, icon: { color: "blue" } });
    }
  }
}

connect(
  "file",
  (data) => {
    console.log(data);

    handleFrameEvent(data);
  }
)
  .then(() => {})
  .catch((e) => console.error(e));
