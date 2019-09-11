import { Intent, Alert, IToaster } from "@blueprintjs/core";
import { byteArrayToString, status } from "./helpers";
import React from "react";
import Websocket from "react-websocket";
import AddressManager, { AddressOptions } from "./AddressManager";

interface AppProps {
  toaster: IToaster;
}

interface AppState {
  connected: boolean;
  addresses: { [id: string]: AddressOptions };
  error?: string;
}

export class App extends React.Component<AppProps, AppState> {
  state: AppState = {
    connected: false,
    addresses: {},
    error: undefined,
  };

  handleFrameEvent(event: FrameEvent) {
    if (event.type === "NewAddress") {
      const id = event.data;

      this.updateAddress(id, {});
    } else if (event.type === "AccessPoint") {
      const [id, info] = event.data;
      const { ssid: ssidBytes, channel } = info;

      this.updateAddress(id, {
        accessPointInfo: { ssidBytes, channel },
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
    } else if (event.type === "InactiveAddress") {
      // const addrs = event.data;
      // const changed: { [id: string]: AddressOptions } = {};
      // addrs.forEach((id) => {
      //   changed[id] = { ...this.state.addresses[id], icon: { size: 25 } };
      // });
      // this.setState({ addresses: { ...this.state.addresses, ...changed } });
    } else if (event.type === "Loss") {
      const [id, numLost, numReceived] = event.data;

      const loss = numLost / (numLost + numReceived);

      this.updateAddress(id, {
        loss,
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

  handleMessage(msg: string) {
    const events: Array<FrameEvent> = JSON.parse(msg);
    events.forEach((event) => this.handleFrameEvent(event));
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

        <AddressManager addresses={addresses} toaster={toaster} />
        <Websocket
          url={`ws://${location.host}/ws`}
          onMessage={(msg: string) => this.handleMessage(msg)}
          onOpen={() => {
            status("websocket opened");
            this.setState({ connected: true });
          }}
          onClose={() => {
            status("websocket closed");
            toaster.show({
              message: "websocket closed",
              intent: "danger",
            });
            this.setState({ connected: false });
          }}
          debug={true}
          reconnect={false}
        />
      </div>
    );
  }
}
