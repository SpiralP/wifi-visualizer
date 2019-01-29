const jsesc: (input: string) => string = require("jsesc");

export function isBroadcast(mac: string): boolean {
  return (parseInt(`${mac[0]}${mac[1]}`, 16) & 0b01) != 0;
}

export function hashMacs(mac1: string, mac2: string): string {
  if (mac1 >= mac2) return `${mac1}${mac2}`;
  else return `${mac2}${mac1}`;
}

const namedTimeouts = {};
export function setNamedTimeout(
  name: string,
  callback: () => void,
  time: number
) {
  if (namedTimeouts[name]) {
    clearTimeout(namedTimeouts[name]);
  }
  namedTimeouts[name] = setTimeout(callback, time);
}

export const iconNameToCode = {
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
  home: "\uf015",
};

// tslint:disable-next-line:object-literal-key-quotes
export const ouiToIconCode = {
  Cisco: iconNameToCode.broadcast_tower,
  Belkin: iconNameToCode.broadcast_tower,
  ASUSTek: iconNameToCode.broadcast_tower,
  ARRIS: iconNameToCode.broadcast_tower,
  "Tp Link": iconNameToCode.broadcast_tower,
  "Texas Instruments": iconNameToCode.broadcast_tower,
  Netgear: iconNameToCode.broadcast_tower,
  "Ruckus Wireless": iconNameToCode.broadcast_tower,
  Aruba: iconNameToCode.broadcast_tower,
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
  Skybell: iconNameToCode.home,
};

export function companyToIconCode(company?: string) {
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

export async function connect(
  kind: string,
  callback: (event: FrameEvent) => void
) {
  const ws = new WebSocket(`ws://localhost:3012/${kind}`);

  ws.onmessage = function message(data) {
    const frameEvent: FrameEvent = JSON.parse(data.data);
    callback(frameEvent);
  };

  // wait for onopen
  await new Promise((resolve, reject) => {
    ws.onerror = function error(err) {
      reject(err);
    };
    ws.onopen = function open() {
      resolve();
    };
  });

  // wait for onclose
  await new Promise((resolve, reject) => {
    ws.onerror = function error(err) {
      reject(err);
    };
    ws.onclose = function close() {
      resolve();
    };
  });
}

export function htmlEscape(input: string): string {
  // TODO
  return input;
}

export function byteArrayToString(input: ByteArray): string {
  return jsesc(Buffer.from(input).toString());
}
