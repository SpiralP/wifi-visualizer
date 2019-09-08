import React from "react";
import ReactDOM from "react-dom";
import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import { status } from "./helpers";
import { App } from "./App";

// @ts-ignore
if (module.hot != null) {
  // @ts-ignore
  module.hot.dispose(() => {
    // module is about to be replaced
    document.location.reload(true);
  });
}

status("index.tsx");

const root = document.getElementById("root");
if (root) {
  ReactDOM.render(<App />, root);
} else {
  console.error("no root element!");
}
