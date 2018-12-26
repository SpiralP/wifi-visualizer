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
