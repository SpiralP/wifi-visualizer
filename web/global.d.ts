declare type MacAddress = string;

declare interface AccessPointFrameKind {
  type: "AccessPoint";
  data: number[];
}
declare interface StationFrameKind {
  type: "Station";
}
declare type FrameKind = AccessPointFrameKind | StationFrameKind;

declare interface NewAddressFrameEvent {
  type: "NewAddress";
  data: MacAddress;
}
declare interface ConnectionFrameEvent {
  type: "Connection";
  data: [MacAddress, MacAddress];
}
declare interface LeaveFrameEvent {
  type: "Leave";
  data: MacAddress;
}
declare interface SetKindFrameEvent {
  type: "SetKind";
  data: [MacAddress, FrameKind];
}

declare type FrameEvent =
  | NewAddressFrameEvent
  | ConnectionFrameEvent
  | LeaveFrameEvent
  | SetKindFrameEvent;
