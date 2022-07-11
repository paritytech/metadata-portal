import { render } from "@testing-library/react";
import App from "./App";
import { BrowserRouter as Router } from "react-router-dom";

test("renders ok", () => {
  render(
    <Router>
      <App />
    </Router>
  );
});

test("data file exists", async () => {
  require("../../public/data.json");
});
