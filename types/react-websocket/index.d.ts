declare module "react-websocket" {
  const Websocket: React.ElementType<{
    /** required The url the websocket connection is listening to. */
    url: string;

    /** required The callback called when data is received. Data is JSON.parse'd */
    onMessage: (message: string) => void;

    /** The callback called when the connection is successfully opened. */
    onOpen: () => void;

    /** The callback called when the connection is closed either due to server disconnect or network error. */
    onClose: () => void;

    /** default: false Set to true to see console logging */
    debug: boolean;

    /** default: true accelerated reconnection time */
    reconnect: boolean;
  }>;
  export default Websocket;
}
