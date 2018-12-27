// TODO import fontawesome for offline

import { hashMacs, isBroadcast, setNamedTimeout } from "./helpers";
import copy from "clipboard-copy";
const jsesc: (input: string) => string = require("jsesc");
import oui from "./oui";
import vis from "vis";

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
  google: "\uf1a0",
  printer: "\uf02f",
  microsoft: "\uf3ca",
  video: "\uf03d",
};

// tslint:disable-next-line:object-literal-key-quotes
const ouiToIconCode = {
  Cisco: iconNameToCode.broadcast_tower,
  Belkin: iconNameToCode.broadcast_tower,
  ASUSTek: iconNameToCode.broadcast_tower,
  ARRIS: iconNameToCode.broadcast_tower,
  "Tp Link": iconNameToCode.broadcast_tower,
  "Texas Instruments": iconNameToCode.broadcast_tower,
  Netgear: iconNameToCode.broadcast_tower,
  Intel: iconNameToCode.desktop,
  LG: iconNameToCode.android,
  Apple: iconNameToCode.apple,
  Murata: iconNameToCode.mobile,
  Samsung: iconNameToCode.android,
  Valve: iconNameToCode.steam,
  Amazon: iconNameToCode.amazon,
  HTC: iconNameToCode.android,
  Roku: iconNameToCode.tv,
  "Hewlett Packard": iconNameToCode.printer,
  "Cisco Linksys": iconNameToCode.broadcast_tower,
  Google: iconNameToCode.google,
  "Nest Labs": iconNameToCode.google,
  "2Wire": iconNameToCode.broadcast_tower,
  "Hon Hai": iconNameToCode.broadcast_tower,
  Microsoft: iconNameToCode.microsoft,
  Technicolor: iconNameToCode.broadcast_tower,
  "Shenzhen Reecam": iconNameToCode.video,
  "ABB/Tropos": iconNameToCode.broadcast_tower,
  BLU: iconNameToCode.android,
  zte: iconNameToCode.android,
  "TCL Technoly": iconNameToCode.tv,
};

function companyToIconCode(company?: string) {
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

function byteArrayToString(input: ByteArray): string {
  return jsesc(Buffer.from(input).toString());
}

function handleFrameEvent(event: FrameEvent) {
  if (event.type === "NewAddress") {
    const mac = event.data;
    const company = oui(mac);

    nodes.add({
      id: mac,
      icon: { code: companyToIconCode(company) },
      hover: true,
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
  } else if (event.type === "NewConnection") {
    const [from, to] = event.data;

    edges.add({
      id: hashMacs(from, to),
      from,
      to,
    });
  } else if (event.type === "RemoveConnection") {
    const [from, to] = event.data;
    edges.remove(hashMacs(from, to));
  } else if (event.type === "ProbeRequest") {
    const [from, ssid] = event.data;
    // console.log(from, byteArrayToString(ssid));
  } else if (event.type === "ProbeResponse") {
    const [from, to, ssid] = event.data;
    // console.log(from, byteArrayToString(ssid));
  }
}

connect(
  "file",
  (data) => {
    // console.log(data);

    handleFrameEvent(data);
  }
)
  .then(() => {})
  .catch((e) => console.error(e));
