const ouiData: {
  [s: string]: undefined | string | { [s: string]: undefined | string };
} = require("./oui_data.json");

export default function oui(mac: string): string | undefined {
  // mac in format 11:22:33:44:55:66
  const cleanMac = mac.replace(/:/g, "");
  const companyOrObject = ouiData[cleanMac.slice(0, 6)];
  if (!companyOrObject) return;

  if (typeof companyOrObject === "string") {
    return companyOrObject;
  } else {
    // object
    for (let i = cleanMac.length; i >= 6; i--) {
      const company = companyOrObject[cleanMac.slice(0, i)];
      if (company) return company;
    }
    return;
  }
}
