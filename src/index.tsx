import React from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./components/App";

// Assert that the element is non-null with `!`
const container = document.getElementById("root")!;
// Now TypeScript knows `container` is not null
const root = createRoot(container);

root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
