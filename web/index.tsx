import start from "./network";
import { status } from "./helpers";

// @ts-ignore
if (module.hot != null) {
  // @ts-ignore
  module.hot.dispose(() => {
    // module is about to be replaced
    document.location.reload(true);
  });
}

status("index.tsx");

setTimeout(() => {
  status("starting");
  start();
}, 1000);
