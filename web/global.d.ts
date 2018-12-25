declare type FrameType = "Management" | "Control" | "Data";

declare type ManagementSubtype =
  | "AssociationRequest"
  | "AssociationResponse"
  | "ReassociationRequest"
  | "ReassociationResponse"
  | "ProbeRequest"
  | "ProbeResponse"
  | "Beacon"
  | "ATIM"
  | "Disassociation"
  | "Authentication"
  | "Deauthentication"
  | "Action";

declare type ControlSubtype =
  | "BlockAckRequest"
  | "BlockAck"
  | "PSPoll"
  | "RTS" // Request To Send
  | "CTS" // Clear To Send
  | "ACK"
  | "CFEnd" // Contention Free
  | "CFEndCFACK";

declare type DataSubtype =
  | "Data"
  | "DataCFAck"
  | "DataCFPoll"
  | "DataCFAckCFPoll"
  | "Null"
  | "CFAck"
  | "CFPoll"
  | "CFAckCFPoll"
  | "QoSData"
  | "QoSDataCFAck"
  | "QoSDataCFPoll"
  | "QoSDataCFAckCFPoll"
  | "QoSNull"
  | "QoSCFPoll = 14" // no data
  | "QoSCFAck"; // no data

declare interface FrameControl {
  version: Version;
  type_: FrameType;
  flags: Flags;
}

declare enum Version {
  Standard,
}

declare interface Flags {
  to_ds: boolean;
  from_ds: boolean;
  more_fragments: boolean;
  retry: boolean;
  pwr_mgt: boolean;
  more_data: boolean;
  protected: boolean;
  order: boolean;
}

declare interface BeaconInfo {
  fixed_parameters: FixedParameters;
  tagged_parameters: TaggedParameters;
}

declare interface TaggedParameters {
  tags: Array<Tag>;
}

declare interface FixedParameters {
  timestamp: number;
  beacon_interval: number; // seconds
  capabilities_info: CapabilitiesInfo;
}

declare interface CapabilitiesInfo {}

declare interface Tag {
  number: number;
  length: number;
  data: Array<number>;
}

type MacAddress = string;

declare interface BasicFrame {
  type: FrameType;
  subtype: ManagementSubtype | ControlSubtype | DataSubtype;

  receiver_address?: MacAddress;
  transmitter_address?: MacAddress;

  destination_address?: MacAddress;
  source_address?: MacAddress;

  bssid?: MacAddress;
}

declare interface ManagementFrame extends BasicFrame {
  type: "Management";
  subtype: ManagementSubtype;
}

declare interface ControlFrame extends BasicFrame {
  type: "Control";
  subtype: ControlSubtype;
}

declare interface DataFrame extends BasicFrame {
  type: "Data";
  subtype: DataSubtype;
}

declare interface BeaconFrame extends ManagementFrame {
  type: "Management";
  subtype: "Beacon";

  beacon_info: BeaconInfo;
}
declare interface OtherManagementFrame extends ManagementFrame {
  subtype: Exclude<ManagementSubtype, "Beacon">;
}

declare type Frame =
  | BeaconFrame
  | OtherManagementFrame
  | ControlFrame
  | DataFrame;
