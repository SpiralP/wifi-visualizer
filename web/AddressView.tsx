import React from "react";
import AddressNetwork, { AddressOptions } from "./AddressNetwork";
import AddressList from "./AddressList";
import { IToaster, Alert, Intent } from "@blueprintjs/core";
import { byteArrayToString } from "./helpers";
import { FrameEvent, MacAddress } from "./interfaceTypes";

interface AddressViewProps {
  toaster: IToaster;
}

interface AddressViewState {
  connected: boolean;
  addresses: { [id: string]: AddressOptions };
  error?: string;
  hovered?: string;
}

export default class AddressView extends React.Component<
  AddressViewProps,
  AddressViewState
> {
  state: AddressViewState = {
    connected: false,
    addresses: {},
    error: undefined,
  };

  public handleFrameEvent(event: FrameEvent) {
    console.log(`handleFrameEvent`, event);

    if (event.type === "NewAddress") {
      const id = event.data;

      this.updateAddress(id, {});
    } else if (event.type === "AccessPoint") {
      const [id, info] = event.data;
      const { ssid: ssidBytes, channel } = info;
      const ssid = byteArrayToString(ssidBytes);

      this.updateAddress(id, {
        accessPointInfo: { ssid, channel },
      });
    } else if (event.type === "Connection") {
      const [from, to, kind] = event.data;

      // TODO editing multiple
      this.setState({
        addresses: {
          ...this.state.addresses,
          [from]: {
            ...this.state.addresses[from],
            connections: {
              ...this.state.addresses[from].connections,
              [to]: kind,
            },
          },
          [to]: {
            ...this.state.addresses[to],
            connections: {
              ...this.state.addresses[to].connections,
              [from]: kind,
            },
          },
        },
      });
    } else if (event.type === "ProbeRequest") {
      const [id, ssidBytes] = event.data;
      const ssid = byteArrayToString(ssidBytes);

      this.updateAddress(id, {
        probeRequests: [
          ...(this.state.addresses[id].probeRequests || []),
          ssid,
        ],
      });
      // } else if (event.type === "Loss") {
      //   const [id, numLost, numReceived] = event.data;

      //   const loss = numLost / (numLost + numReceived);

      //   this.updateAddress(id, {
      //     loss,
      //   });
    } else if (event.type === "Signal") {
      const [id, signal] = event.data;

      this.updateAddress(id, {
        signal,
      });
    } else if (event.type === "Rate") {
      const [id, rate] = event.data;

      this.updateAddress(id, {
        rate,
      });
    } else if (event.type === "Error") {
      const error = event.data;
      console.warn("Error", error);
      this.setState({ error });
    } else {
      console.warn(event);
    }
  }

  updateAddress(id: MacAddress, options: AddressOptions) {
    this.setState({
      addresses: {
        ...this.state.addresses,
        [id]: {
          connections: {},
          probeRequests: [],
          ...this.state.addresses[id],
          ...options,
        },
      },
    });
  }

  render() {
    const { toaster } = this.props;
    const { addresses, error } = this.state;

    return (
      <div>
        <Alert
          isOpen={error ? true : false}
          icon="error"
          intent={Intent.DANGER}
          confirmButtonText="Okay"
          canOutsideClickCancel={true}
          onClose={() => {
            this.setState({ error: undefined });
          }}
        >
          <p>
            Error: <b>{error ? error : "<unknown>"}</b>
          </p>
        </Alert>

        <div>
          <div style={{ position: "absolute", zIndex: 9 }}>
            <AddressList
              addresses={addresses}
              toaster={toaster}
              onAddressHover={(id, hovered) => {
                console.log("hovered", id, hovered);
                this.updateAddress(id, { hovered });
              }}
            />
          </div>

          <AddressNetwork addresses={addresses} toaster={toaster} />
        </div>
      </div>
    );
  }
}
