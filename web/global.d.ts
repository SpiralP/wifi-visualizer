declare type MacAddress = string;

declare type FrameKind = "AccessPoint" | "Station";

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
