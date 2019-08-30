import jsesc from "jsesc";

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
  amazon: "\uf270",
  android: "\uf17b",
  apple: "\uf179",
  broadcast_tower: "\uf519",
  circle: "\uf111",
  desktop: "\uf109",
  gamepad: "\uf11b",
  google: "\uf1a0",
  home: "\uf015",
  lightbulb: "\uf0eb",
  microsoft: "\uf3ca",
  mobile: "\uf10b",
  printer: "\uf02f",
  steam: "\uf1b6",
  tv: "\uf26c",
  video: "\uf03d",
};

// tslint:disable-next-line:object-literal-key-quotes
export const ouiToIconCode = {
  "2Wire": iconNameToCode.broadcast_tower,
  "ABB/Tropos": iconNameToCode.broadcast_tower,
  "Cisco Linksys": iconNameToCode.broadcast_tower,
  "Hewlett Packard": iconNameToCode.printer,
  "Hon Hai": iconNameToCode.broadcast_tower,
  "Lifi Labs Management Pty": iconNameToCode.lightbulb,
  "Lifi Labs": iconNameToCode.lightbulb,
  "Motorola Mobility": iconNameToCode.mobile,
  "Nest Labs": iconNameToCode.google,
  "Ruckus Wireless": iconNameToCode.broadcast_tower,
  "Shenzhen Reecam": iconNameToCode.video,
  "Sony Mobile": iconNameToCode.mobile,
  "TCL Technoly": iconNameToCode.tv,
  "Texas Instruments": iconNameToCode.broadcast_tower,
  "Tp Link": iconNameToCode.broadcast_tower,
  Amazon: iconNameToCode.amazon,
  Apple: iconNameToCode.apple,
  ARRIS: iconNameToCode.broadcast_tower,
  Aruba: iconNameToCode.broadcast_tower,
  ASUSTek: iconNameToCode.broadcast_tower,
  Belkin: iconNameToCode.broadcast_tower,
  BLU: iconNameToCode.android,
  Cisco: iconNameToCode.broadcast_tower,
  Google: iconNameToCode.google,
  HTC: iconNameToCode.android,
  HUMAX: iconNameToCode.tv,
  Intel: iconNameToCode.desktop,
  LG: iconNameToCode.android,
  Microsoft: iconNameToCode.microsoft,
  Murata: iconNameToCode.mobile,
  Netgear: iconNameToCode.broadcast_tower,
  Nintendo: iconNameToCode.gamepad,
  Roku: iconNameToCode.tv,
  Samsung: iconNameToCode.android,
  Skybell: iconNameToCode.home,
  Technicolor: iconNameToCode.broadcast_tower,
  Valve: iconNameToCode.steam,
  Vizio: iconNameToCode.tv,
  zte: iconNameToCode.android,
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

export async function connect(callback: (event: FrameEvent) => void) {
  const ws = new WebSocket(`ws://localhost:8001/`);

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

export function byteArrayToString(input: ByteArray): string {
  return jsesc(Buffer.from(input).toString());
}
