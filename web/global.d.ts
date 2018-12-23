declare type SigmaSettings = SigmaSettingsRenderer &
  SigmaSettingsGraph &
  SigmaSettingsRendererHoverNode &
  SigmaSettingsRendererHoverEdge &
  SigmaSettingsRendererSwitches &
  SigmaSettingsRendererPerformance &
  SigmaSettingsRescale &
  SigmaSettingsCaptors &
  SigmaSettingsGlobal &
  SigmaSettingsCamera &
  SigmaSettingsAnimation;

declare interface SigmaGraphData {
  nodes: Array<SigmaNode>;
  edges: Array<SigmaEdge>;
}

declare interface SigmaNode {
  id: string;
  label?: string;
  x?: number;
  y?: number;
  size?: number;
  color?: color;
}

declare interface SigmaEdge {
  id: string;
  source: string;
  target: string;
  label?: string;
  color?: color;
}

declare interface SigmaNeo4jCypherProducers {
  node: (arg0: Neo4jNode) => SigmaNode;
  edge: (arg1: Neo4jEdge) => SigmaEdge;
}

interface Neo4jNode {
  id: string;
  labels: Array<string>;
  properties: {};
}

interface Neo4jEdge {
  id: string;
  type: string;
  startNode: string;
  endNode: string;
  properties: {};
}

declare type SigmaRenderer = "webgl" | "canvas";

// Error handler function for Sigma component
declare type SigmaErrorHandler = (error: Error) => void;

declare interface SigmaEvent {
  data: {
    node?: Neo4jNode;
    edge?: Neo4jEdge;
    captor: {
      clientX: number;
      clientY: number;
    };
  };
}

// Event handler function for Sigma component
declare type SigmaEventHandler = (node: SigmaEvent) => void;

// Following type requires EdgeShapes component
declare type SigmaEdgeShapes =
  | "def"
  | "line"
  | "arrow"
  | "curved"
  | "curvedArrow"
  | "dashed"
  | "dotted"
  | "parallel"
  | "tapered";

// Following type requires NodeShapes component
declare type SigmaNodeShapes =
  | "def"
  | "pacman"
  | "star"
  | "equilateral"
  | "cross"
  | "diamond"
  | "circle"
  | "square";

// Following type used in Filter component
type NodesFilter = (node: SigmaNode) => boolean;

declare type SigmaEasing =
  | "linear"
  | "quadraticIn"
  | "quadraticOut"
  | "quadraticInOut"
  | "cubicIn"
  | "cubicOut"
  | "cubicInOut";

declare interface SigmaSettingsGraph {
  clone?: boolean;
  immutable?: boolean;
  verbose?: boolean;
}

declare interface SigmaSettingsRenderer {
  defaultNodeType?: string;
  defaultEdgeType?: string;
  defaultLabelColor?: color;
  defaultEdgeColor?: color;
  defaultNodeColor?: color;
  defaultLabelSize?: number;
  edgeColor?: "source" | "target" | "default";
  minArrowSize?: number;
  font?: string;
  fontStyle?: string;
  labelColor?: SigmaDefaultNodeOption;
  labelSize?: "fixed" | "proportional";
  labelSizeRatio?: number;
  labelThreshold?: number;
  webglOversamplingRatio?: number;
}

declare interface SigmaSettingsRendererHoverNode {
  borderSize?: number;
  defaultNodeBorderColor?: color;
  hoverFont?: string;
  hoverFontStyle?: string;
  labelHoverShadow?: SigmaDefaultNodeOption;
  labelHoverShadowColor?: color;
  nodeHoverColor?: SigmaDefaultNodeOption;
  defaultNodeHoverColor?: color;
  labelHoverBGColor?: SigmaDefaultNodeOption;
  defaultHoverLabelBGColor?: color;
  labelHoverColor?: SigmaDefaultNodeOption;
  defaultLabelHoverColor?: color;
  singleHover?: boolean;
}

declare interface SigmaSettingsRendererHoverEdge {
  edgeHoverColor?: color;
  defaultEdgeHoverColor?: SigmaDefaultEdgeOption;
  edgeHoverSizeRatio?: number;
  edgeHoverExtremities?: boolean;
}

declare interface SigmaSettingsRendererSwitches {
  drawLabels?: boolean;
  drawEdgeLabels?: boolean;
  drawEdges?: boolean;
  drawNodes?: boolean;
}

declare interface SigmaSettingsRendererPerformance {
  batchEdgesDrawing?: boolean;
  canvasEdgesBatchSize?: number;
  webglEdgesBatchSize?: number;
  hideEdgesOnMove?: boolean;
}

declare interface SigmaSettingsRescale {
  scalingMode?: "inside" | "outside";
  sideMargin?: number;
  minEdgeSize?: number;
  maxEdgeSize?: number;
  minNodeSize?: number;
  maxNodeSize?: number;
}

declare interface SigmaSettingsCaptors {
  touchEnabled?: boolean;
  mouseEnabled?: boolean;
  mouseWheelEnabled?: boolean;
  doubleClickEnabled?: boolean;
  eventsEnabled?: boolean;
  zoomingRatio?: number;
  doubleClickZoomingRatio?: number;
  zoomMin?: number;
  zoomMax?: number;
  mouseZoomDuration?: number;
  doubleClickZoomDuration?: number;
  mouseInertiaDuration?: number;
  mouseInertiaRatio?: number;
  touchInertiaDuration?: number;
  touchInertiaRatio?: number;
  doubleClickTimeout?: number;
  doubleTapTimeout?: number;
  dragTimeout?: number;
}

declare interface SigmaSettingsGlobal {
  autoResize?: boolean;
  autoRescale?: boolean;
  enableCamera?: boolean;
  enableHovering?: boolean;
  enableEdgeHovering?: boolean;
  edgeHoverPrecision?: number;
  rescaleIgnoreSize?: boolean;
  skipErrors?: boolean;
}

declare interface SigmaSettingsCamera {
  nodesPowRatio?: number;
  edgesPowRatio?: number;
}

declare interface SigmaSettingsAnimation {
  animationsTime?: number;
}

declare type SigmaDefaultNodeOption = "default" | "node";

declare type SigmaDefaultEdgeOption = "default" | "node";

type color = string;

declare enum Version {
  Standard,
}

declare enum ManagementSubtype {
  AssociationRequest,
  AssociationResponse,
  ReassociationRequest,
  ReassociationResponse,
  ProbeRequest,
  ProbeResponse,
  // 6-7 Reserved
  Beacon = 8,
  ATIM,
  Disassociation,
  Authentication,
  Deauthentication,
  Action,
  // 14-15 Reserved
}

declare enum ControlSubtype {
  // 0-7 Reserved
  BlockAckRequest = 8,
  BlockAck,
  PSPoll,
  RTS, // Request To Send
  CTS, // Clear To Send
  ACK,
  CFEnd, // Contention Free
  CFEndCFACK,
}

declare enum DataSubtype {
  Data,
  DataCFAck,
  DataCFPoll,
  DataCFAckCFPoll,
  Null,
  CFAck,
  CFPoll,
  CFAckCFPoll,
  QoSData,
  QoSDataCFAck,
  QoSDataCFPoll,
  QoSDataCFAckCFPoll,
  QoSNull,
  // 13 Reserved
  QoSCFPoll = 14, // no data
  QoSCFAck, // no data
}

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

  receiver_address: MacAddress;
  transmitter_address: MacAddress;

  destination_address: MacAddress;
  source_address: MacAddress;

  bssid?: MacAddress;
}

declare interface BeaconFrame extends BasicFrame {
  fragment_number: number;
  sequence_number: number;
}

declare interface Frame {
  Basic?: BasicFrame;
  Beacon?: BeaconFrame;
}
