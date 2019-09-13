import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import "vis-network/dist/vis-network.css"; // for popups
import "@fortawesome/fontawesome-free/css/solid.css";
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import React from "react";
import { storiesOf } from "@storybook/react";
import AddressNetwork, { AddressOptions } from "../web/AddressNetwork";
import { Toaster } from "@blueprintjs/core";

const toaster = Toaster.create({ position: "top-right" });

storiesOf("AddressNetwork", module)
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

    return <AddressNetwork addresses={addresses} toaster={toaster} />;
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
              ssid: "ssid hello",
            },
            connections: { ["98-d6-f7-01-01-01"]: "Associated" },
          },
        };

        for (let i = 0; i < 100; i++) {
          addresses["" + i] = { connections: { ["" + (i - 1)]: "InRange" } };
        }

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
          a("98-d6-f7-01-01-00", { signal: -20 });

          for (let jkjk = 0; jkjk < 5; jkjk++) {
            for (let i = 10; i < 100; i++) {
              await t(10);
              a("98-d6-f7-01-01-00", { signal: -1 * i });
              a("98-d6-f7-01-01-01", { signal: -1 * i });
            }
          }
        })();
      }

      render() {
        return (
          <AddressNetwork addresses={this.state.addresses} toaster={toaster} />
        );
      }
    }

    return <Changing />;
  });
