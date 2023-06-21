import "@blueprintjs/core/lib/css/blueprint.css";
import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import "@fortawesome/fontawesome-free/css/solid.css";
import "vis-network/dist/dist/vis-network.css"; // for popups
// import "@fortawesome/fontawesome-free/css/regular.css"; // adding regular will break all solid
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";

import { Toaster } from "@blueprintjs/core";
import React from "react";
import ReactDOM from "react-dom";
import { App } from "./App";
import { status } from "./helpers";

// @ts-ignore
if (module.hot != null) {
  // @ts-ignore
  module.hot.dispose(() => {
    // module is about to be replaced
    document.location.reload();
  });
}

status("index.tsx");

const toaster = Toaster.create({ position: "top-right" });
ReactDOM.render(<App toaster={toaster} />, document.getElementById("root"));
