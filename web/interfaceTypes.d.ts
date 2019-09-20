export type MacAddress = string;
export type ByteArray = number[];

export interface AccessPointInfo {
  ssid: ByteArray;
  channel?: number;
}

export type ConnectionType =
  | "Associated"
  | "Authentication"
  | "Disassociated"
  | "InRange";

export interface FrameEventPrototype {
  type: string;
  data: any;
}
export interface NewAddressFrameEvent extends FrameEventPrototype {
  type: "NewAddress";
  data: MacAddress;
}
export interface AccessPointFrameEvent extends FrameEventPrototype {
  type: "AccessPoint";
  data: [MacAddress, AccessPointInfo];
}
export interface ConnectionFrameEvent extends FrameEventPrototype {
  type: "Connection";
  data: [MacAddress, MacAddress, ConnectionType];
}
export interface ProbeRequestFrameEvent extends FrameEventPrototype {
  type: "ProbeRequest";
  data: [MacAddress, ByteArray];
}
// export interface InactiveAddressFrameEvent extends FrameEventPrototype {
//   type: "InactiveAddress";
//   data: MacAddress[];
// }
// export interface LossFrameEvent extends FrameEventPrototype {
//   type: "Loss";
//   /**
//    * addr, # lost, # received
//    */
//   data: [MacAddress, number, number];
// }
export interface SignalEvent extends FrameEventPrototype {
  type: "Signal";
  data: [MacAddress, number];
}
export interface RateEvent extends FrameEventPrototype {
  type: "Rate";
  data: [MacAddress, number];
}
export interface BeaconQualityEvent extends FrameEventPrototype {
  type: "BeaconQuality";
  // #received, #correct
  data: [MacAddress, number, number];
}
export interface ErrorFrameEvent extends FrameEventPrototype {
  type: "Error";
  data: string;
}

export type FrameEvent =
  | NewAddressFrameEvent
  | AccessPointFrameEvent
  | ConnectionFrameEvent
  | ProbeRequestFrameEvent
  // | InactiveAddressFrameEvent
  // | LossFrameEvent
  | SignalEvent
  | RateEvent
  | BeaconQualityEvent
  | ErrorFrameEvent;
