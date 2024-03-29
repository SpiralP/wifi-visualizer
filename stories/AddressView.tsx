import "@blueprintjs/core/lib/css/blueprint.css";
import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@fortawesome/fontawesome-free/css/solid.css";
import "vis-network/dist/dist/vis-network.css"; // for popups
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import { Toaster } from "@blueprintjs/core";
import { storiesOf } from "@storybook/react";
import React from "react";
import AddressView from "../web/AddressView";
import { FrameEvent } from "../web/interfaceTypes";

const toaster = Toaster.create({ position: "top-right" });

storiesOf("AddressView", module)
  .addParameters({ options: { showPanel: false } })
  .add("With Data", () => {
    // const addresses: { [id: string]: AddressOptions } = {
    //   ["98-d6-f7-01-01-00"]: {
    //     connections: { ["98-d6-f7-01-01-02"]: "InRange" },
    //     signal: -20,
    //   },
    //   ["98-d6-f7-01-01-01"]: {
    //     connections: { ["98-d6-f7-01-01-02"]: "Associated" },
    //     signal: -10,
    //   },
    //   ["98-d6-f7-01-01-04"]: {
    //     connections: { ["98-d6-f7-01-01-02"]: "Associated" },
    //     signal: -10,
    //   },
    //   ["98-d6-f7-01-01-03"]: {
    //     connections: { ["98-d6-f7-01-01-02"]: "Associated" },
    //   },
    //   ["98-d6-f7-01-01-02"]: {
    //     accessPointInfo: {
    //       channel: 1,
    //       ssid: "ssid hello",
    //     },
    //     connections: { ["98-d6-f7-01-01-01"]: "Associated" },
    //     signal: -80,
    //   },
    // };

    return (
      <AddressView
        toaster={toaster}
        ref={(addressView) => {
          if (!addressView) {
            throw new Error("no ref addressView?");
          }

          const events: Array<FrameEvent> = [
            {
              type: "NewAddress",
              data: "98-d6-f7-01-01-00",
            },
            {
              type: "NewAddress",
              data: "98-d6-f7-01-01-01",
            },
            {
              type: "NewAddress",
              data: "98-d6-f7-01-01-02",
            },
            {
              type: "NewAddress",
              data: "98-d6-f7-01-01-03",
            },
            {
              type: "Signal",
              data: ["98-d6-f7-01-01-00", -20],
            },
            {
              type: "Signal",
              data: ["98-d6-f7-01-01-01", -10],
            },
            {
              type: "AccessPoint",
              data: [
                "98-d6-f7-01-01-02",
                {
                  channel: 1,
                  ssid: Buffer.from("ssid hello").toJSON().data,
                },
              ],
            },
            {
              type: "Connection",
              data: ["98-d6-f7-01-01-00", "98-d6-f7-01-01-02", "InRange"],
            },
            {
              type: "Connection",
              data: ["98-d6-f7-01-01-01", "98-d6-f7-01-01-02", "Associated"],
            },
            {
              type: "Connection",
              data: ["98-d6-f7-01-01-01", "98-d6-f7-01-01-03", "InRange"],
            },
            {
              type: "AccessPoint",
              data: [
                "98-d6-f7-01-01-03",
                {
                  channel: 1,
                  ssid: Buffer.from("ssid hello2").toJSON().data,
                },
              ],
            },
            {
              type: "BeaconQuality",
              data: ["98-d6-f7-01-01-03", 0.5],
            },
            {
              type: "BeaconQuality",
              data: ["98-d6-f7-01-01-03", 0.8],
            },
            {
              type: "BeaconQuality",
              data: ["98-d6-f7-01-01-03", 0.9],
            },
            {
              type: "BeaconQuality",
              data: ["98-d6-f7-01-01-03", 1.0],
            },
          ];

          const t = (n: number) =>
            new Promise((a) => {
              setTimeout(a, n);
            });

          (async () => {
            for (const event of events) {
              await t(100);
              addressView.handleFrameEvent(event);
            }
          })();
        }}
      />
    );
  });
