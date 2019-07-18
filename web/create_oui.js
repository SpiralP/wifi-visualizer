const fs = require("fs");
const https = require("https");

new Promise((resolve, reject) => {
  console.log("get");

  https
    .get(
      "https://code.wireshark.org/review/gitweb?p=wireshark.git;a=blob_plain;f=manuf;hb=HEAD",
      (res) => {
        const { statusCode } = res;
        if (statusCode !== 200) {
          reject(new Error(`status code ${statusCode}`));
        }

        const data = [];
        res.on("data", (d) => {
          data.push(d);
        });
        res.on("end", () => {
          resolve(data.join(""));
        });
      }
    )
    .on("error", reject);
})
  .then((data) => {
    console.log("process");

    const obj = {};
    // { [first6]: { [first6]: "bap", [first7]: "ag" } }

    data.split("\n").forEach((line) => {
      if (line.match("^#")) return;
      const [mac, shortName, longName] = line.split("\t");
      const name = longName || shortName;
      if (!(mac && name)) return;

      // 00:50:C2:00:10:00/36
      // 00:11:22
      let cleanMac = mac.replace(/:/g, "");
      const first6 = cleanMac.slice(0, 6);
      if (!obj[first6]) obj[first6] = {};
      const sub = obj[first6];

      if (cleanMac.match("/")) {
        // has a netmask
        let [addr, netmask] = cleanMac.split("/");
        cleanMac = addr.slice(0, parseInt(netmask) / 4);
      }

      // Samsung Electro-Mechanics(Thailand)
      // LG Electronics (Mobile Communications)
      // Compal Information (Kunshan) Co., Ltd.
      // Hon Hai Precision Ind. Co.,Ltd.
      // DURATECH Enterprise,LLC
      let cleanName = name;
      while (true) {
        const newCleanName = cleanName
          .replace(/Co$/, "")
          .replace(/Ltd$/, "")
          .replace(/Inc$/, "")
          .replace(/INC$/, "")
          .replace(/Ind$/, "")
          .replace(/LLC$/, "")
          .replace(/ ltd$/, "")

          // specific cases
          .replace(/ CH USA$/i, "")
          .replace(/\(Kunshan\)$/i, "")
          .replace(/\(Huizhou\)$/i, "")
          .replace(/\(Shanghai\)$/i, "")
          .replace(/\(Thailand\)$/i, "")
          .replace(/\(Mobile Communications\)$/i, "")
          .replace(/ Electro\-Mechanics$/i, "")
          .replace(/ COMPUTER$/i, "")
          .replace(/ Inc\./, "")
          .replace(/, a Hewlett Packard Enterprise Company/, "")
          .replace(/ LLC, a Lenovo Company$/, "")
          .replace(/ Management Pty$/, "")

          .replace(/ Corporate$/i, "")
          .replace(/ Corporation$/i, "")
          .replace(/ Tech$/i, "")
          .replace(/ Technology$/i, "")
          .replace(/ Technologies$/i, "")
          .replace(/ Electronics$/i, "")
          .replace(/ International$/i, "")
          .replace(/ Information$/i, "")
          .replace(/ Communications$/i, "")
          .replace(/ Precision$/i, "")
          .replace(/ Group$/i, "")
          .replace(/ Enterprise$/i, "")
          .replace(/ Manufacturing$/i, "")
          .replace(/ Systems$/i, "")
          .replace(/ Products$/i, "")

          .replace(/\,+$/, "")
          .replace(/\.+$/, "")
          .trim();

        if (newCleanName !== cleanName) {
          cleanName = newCleanName;
        } else {
          break;
        }
      }

      cleanName = cleanName.replace("-", " ").replace(/"/g, '\\"');

      sub[cleanMac] = cleanName;
    });

    Object.entries(obj).forEach(([key, value]) => {
      if (Object.keys(value).length === 1) obj[key] = value[key];
    });

    fs.writeFileSync("./oui_data.json", JSON.stringify(obj));
  })
  .catch(console.error);
