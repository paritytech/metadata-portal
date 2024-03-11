import { render } from "@testing-library/react";
import App from "./App";

test("renders ok", () => {
  render(
      <App />
  );
});

test("data file exists", async () => {
  require("../chains.json");
});
