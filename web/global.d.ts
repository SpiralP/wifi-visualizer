declare enum Version {
  Standard,
}

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

declare interface Type {
  Management?: ManagementSubtype;
  Control?: ControlSubtype;
  Data?: DataSubtype;
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

declare interface FrameControl {
  version: Version;
  type_: Type;
  flags: Flags;
}

type MacAddress = string;

declare interface BasicFrame {
  frame_control: FrameControl;
  duration: number; // microseconds

  receiver_address?: MacAddress;
  transmitter_address?: MacAddress;

  destination_address?: MacAddress;
  source_address?: MacAddress;

  bssid?: MacAddress;
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

declare interface BeaconFrame extends BasicFrame {
  fragment_number: number;
  sequence_number: number;

  beacon_info: BeaconInfo;
}

declare interface Frame {
  Basic?: BasicFrame;
  Beacon?: BeaconFrame;
}
