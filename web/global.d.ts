declare type MacAddress = string;
declare type ByteArray = number[];

declare interface AccessPointFrameKind {
  type: "AccessPoint";
  data: ByteArray;
}
declare interface StationFrameKind {
  type: "Station";
}
declare type FrameKind = AccessPointFrameKind | StationFrameKind;

declare type ConnectionType = "Associated" | "Disassociated" | "InRange";

declare interface FrameEventPrototype {
  type: string;
  data: any;
  t: number; // milliseconds from Date.now()
}
declare interface NewAddressFrameEvent extends FrameEventPrototype {
  type: "NewAddress";
  data: MacAddress;
}
declare interface SetKindFrameEvent extends FrameEventPrototype {
  type: "SetKind";
  data: [MacAddress, FrameKind];
}
declare interface ConnectionFrameEvent extends FrameEventPrototype {
  type: "Connection";
  data: [MacAddress, MacAddress, ConnectionType];
}
declare interface ProbeRequestFrameEvent extends FrameEventPrototype {
  type: "ProbeRequest";
  data: [MacAddress, ByteArray];
}

declare type FrameEvent =
  | NewAddressFrameEvent
  | SetKindFrameEvent
  | ConnectionFrameEvent
  | ProbeRequestFrameEvent;
