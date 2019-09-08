import * as React from "react";

import { storiesOf } from "@storybook/react";
import { Network } from "../web/Network";
import { Toaster } from "@blueprintjs/core";

const toaster = Toaster.create();
storiesOf("Network", module).add("Nodes", () => (
  <Network nodes={{}} edges={{}} toaster={toaster} />
));
