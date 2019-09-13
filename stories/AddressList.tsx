import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import "vis-network/dist/vis-network.css"; // for popups
import "@fortawesome/fontawesome-free/css/solid.css";
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import React from "react";
import { storiesOf } from "@storybook/react";
import AddressList from "../web/AddressList";
import { AddressOptions } from "../web/AddressNetwork";
import { Toaster } from "@blueprintjs/core";

const toaster = Toaster.create();

storiesOf("AddressList", module)
  .addParameters({ options: { showPanel: false } })
  .add("With Data", () => {
    const addresses: { [id: string]: AddressOptions } = {
      ["98-d6-f7-01-01-00"]: {
        connections: { ["98-d6-f7-01-01-02"]: "InRange" },
        signal: -20,
      },
      ["98-d6-f7-01-01-01"]: {
        connections: { ["98-d6-f7-01-01-02"]: "Associated" },
        signal: -10,
      },
      ["98-d6-f7-01-01-04"]: {
        connections: { ["98-d6-f7-01-01-02"]: "Associated" },
        signal: -10,
      },
      ["98-d6-f7-01-01-03"]: {
        connections: { ["98-d6-f7-01-01-02"]: "Associated" },
      },
      ["98-d6-f7-01-01-02"]: {
        accessPointInfo: {
          channel: 1,
          ssid: "ssid hello",
        },
        connections: { ["98-d6-f7-01-01-01"]: "Associated" },
        signal: -80,
      },
    };

    return (
      <AddressList
        addresses={addresses}
        toaster={toaster}
        onAddressHover={() => {}}
      />
    );
  });
