declare type MacAddress = string;
declare type ByteArray = number[];

declare interface AccessPointInfo {
  ssid: ByteArray;
  channel?: number;
}

declare type ConnectionType =
  | "Associated"
  | "Authentication"
  | "Disassociated"
  | "InRange";

declare interface FrameEventPrototype {
  type: string;
  data: any;
}
declare interface NewAddressFrameEvent extends FrameEventPrototype {
  type: "NewAddress";
  data: MacAddress;
}
declare interface AccessPointFrameEvent extends FrameEventPrototype {
  type: "AccessPoint";
  data: [MacAddress, AccessPointInfo];
}
declare interface ConnectionFrameEvent extends FrameEventPrototype {
  type: "Connection";
  data: [MacAddress, MacAddress, ConnectionType];
}
declare interface ProbeRequestFrameEvent extends FrameEventPrototype {
  type: "ProbeRequest";
  data: [MacAddress, ByteArray];
}
// declare interface InactiveAddressFrameEvent extends FrameEventPrototype {
//   type: "InactiveAddress";
//   data: MacAddress[];
// }
// declare interface LossFrameEvent extends FrameEventPrototype {
//   type: "Loss";
//   /**
//    * addr, # lost, # received
//    */
//   data: [MacAddress, number, number];
// }
declare interface SignalEvent extends FrameEventPrototype {
  type: "Signal";
  data: [MacAddress, number];
}
declare interface RateEvent extends FrameEventPrototype {
  type: "Rate";
  data: [MacAddress, number];
}
declare interface ErrorFrameEvent extends FrameEventPrototype {
  type: "Error";
  data: string;
}

declare type FrameEvent =
  | NewAddressFrameEvent
  | AccessPointFrameEvent
  | ConnectionFrameEvent
  | ProbeRequestFrameEvent
  // | InactiveAddressFrameEvent
  // | LossFrameEvent
  | SignalEvent
  | RateEvent
  | ErrorFrameEvent;
