import start from "./network";

// @ts-ignore
if (module.hot != null) {
  // @ts-ignore
  module.hot.dispose(() => {
    // module is about to be replaced
    document.location.reload(true);
  });
}

setTimeout(() => {
  start();
}, 0);
