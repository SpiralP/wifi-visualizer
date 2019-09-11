import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import "vis-network/dist/vis-network.css"; // for popups
import "@fortawesome/fontawesome-free/css/solid.css";
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import React from "react";
import { storiesOf } from "@storybook/react";
import AddressManager, { AddressOptions } from "../web/AddressManager";
import { Toaster } from "@blueprintjs/core";

const toaster = Toaster.create();

storiesOf("AddressManager", module)
  .addParameters({ options: { showPanel: false } })
  .add("With Data", () => {
    const addresses: { [id: string]: AddressOptions } = {
      ["98-d6-f7-01-01-00"]: {
        connections: { ["98-d6-f7-01-01-02"]: "InRange" },
      },
      ["98-d6-f7-01-01-01"]: {
        connections: { ["98-d6-f7-01-01-02"]: "Associated" },
      },
      ["98-d6-f7-01-01-02"]: {
        accessPointInfo: {
          channel: 1,
          ssidBytes: Buffer.from("ssid hello").toJSON().data,
        },
        connections: { ["98-d6-f7-01-01-01"]: "Associated" },
      },
    };

    return <AddressManager addresses={addresses} toaster={toaster} />;
  })
  .add("Changing", () => {
    class Changing extends React.PureComponent<
      {},
      { addresses: { [id: string]: AddressOptions } }
    > {
      constructor(props) {
        super(props);

        const addresses: { [id: string]: AddressOptions } = {
          ["98-d6-f7-01-01-00"]: {
            connections: { ["98-d6-f7-01-01-02"]: "InRange" },
          },
          ["98-d6-f7-01-01-01"]: {
            connections: { ["98-d6-f7-01-01-02"]: "Associated" },
          },
          ["98-d6-f7-01-01-02"]: {
            accessPointInfo: {
              channel: 1,
              ssidBytes: Buffer.from("ssid hello").toJSON().data,
            },
            connections: { ["98-d6-f7-01-01-01"]: "Associated" },
          },
        };

        this.state = { addresses };
      }

      componentDidMount() {
        const a = (id: string, b: AddressOptions) => {
          const addresses = { ...this.state.addresses };

          addresses[id] = { ...addresses[id], ...b };

          this.setState({
            addresses,
          });
        };

        const t = (n: number) =>
          new Promise((a) => {
            setTimeout(a, n);
          });

        (async () => {
          await t(1000);
          a("98-d6-f7-01-01-00", { loss: 0.45 });

          for (let i = 0; i < 100; i++) {
            await t(10);
            a("98-d6-f7-01-01-00", { loss: i / 100 });
          }
        })();
      }

      render() {
        return (
          <AddressManager addresses={this.state.addresses} toaster={toaster} />
        );
      }
    }

    return <Changing />;
  });
