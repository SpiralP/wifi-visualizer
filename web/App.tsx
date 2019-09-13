import { IToaster } from "@blueprintjs/core";
import { status } from "./helpers";
import React from "react";
import Websocket from "react-websocket";
import AddressView from "./AddressView";
import { FrameEvent } from "./interfaceTypes";

interface AppProps {
  toaster: IToaster;
}

interface AppState {
  connected: boolean;
}

export class App extends React.Component<AppProps, AppState> {
  state: AppState = {
    connected: false,
  };

  addressView: AddressView | null;

  handleMessage(msg: string) {
    const events: Array<FrameEvent> = JSON.parse(msg);
    events.forEach((event) => {
      if (!this.addressView) {
        throw new Error("no this.addressView?");
      }
      this.addressView.handleFrameEvent(event);
    });
  }

  render() {
    const { toaster } = this.props;
    // const {connected} = this.state;

    return (
      <div>
        <AddressView
          toaster={toaster}
          ref={(addressView) => {
            this.addressView = addressView;
          }}
        />

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
              timeout: 10000,
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
