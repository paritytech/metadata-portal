import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import App, { ChainsMode } from "./components/App";

ReactDOM.render(
  <Router>
    <React.StrictMode>
      <Routes>
        <Route
          path="/"
          element={<App mode={ChainsMode.Prod} />}
        />
        <Route
          path="/dev"
          element={<App mode={ChainsMode.Dev} />}
        />
      </Routes>
    </React.StrictMode>
  </Router>,
  document.getElementById("root")
);
