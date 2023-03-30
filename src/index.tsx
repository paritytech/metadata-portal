import React from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import App, { ChainsMode } from "./components/App";
import "./index.css";

createRoot(document.getElementById("root") as HTMLElement).render(
  <Router>
    <React.StrictMode>
      <Routes>
        <Route path="/" element={<App mode={ChainsMode.Prod} />} />
        <Route path="/dev" element={<App mode={ChainsMode.Dev} />} />
      </Routes>
    </React.StrictMode>
  </Router>
);