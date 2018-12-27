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

declare interface NewAddressFrameEvent {
  type: "NewAddress";
  data: MacAddress;
}
declare interface SetKindFrameEvent {
  type: "SetKind";
  data: [MacAddress, FrameKind];
}
declare interface NewConnectionFrameEvent {
  type: "NewConnection";
  data: [MacAddress, MacAddress];
}
declare interface RemoveConnectionFrameEvent {
  type: "RemoveConnection";
  data: [MacAddress, MacAddress];
}
declare interface ProbeRequestFrameEvent {
  type: "ProbeRequest";
  data: [MacAddress, ByteArray];
}
declare interface ProbeResponseFrameEvent {
  type: "ProbeResponse";
  data: [MacAddress, MacAddress, ByteArray];
}

declare type FrameEvent =
  | NewAddressFrameEvent
  | SetKindFrameEvent
  | NewConnectionFrameEvent
  | RemoveConnectionFrameEvent
  | ProbeRequestFrameEvent
  | ProbeResponseFrameEvent;
