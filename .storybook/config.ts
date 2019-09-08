import { configure } from "@storybook/react";

// automatically import all files ending in *.stories.tsx
const req = require.context("../stories", true, /\.tsx$/);
console.log(req);
function loadStories() {
  console.log(req);
  req.keys().forEach((filename) => req(filename));
}

configure(loadStories, module);
