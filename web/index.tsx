import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@blueprintjs/core/lib/css/blueprint.css";
import "vis-network/dist/vis-network.css"; // for popups
import "@fortawesome/fontawesome-free/css/solid.css";
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import React from "react";
import ReactDOM from "react-dom";
import { status } from "./helpers";
import { Toaster } from "@blueprintjs/core";
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
  const toaster = Toaster.create({ position: "top-right" });
  ReactDOM.render(<App toaster={toaster} />, root);
} else {
  console.error("no root element!");
}
