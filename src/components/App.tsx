import { Routes, Route, HashRouter } from "react-router-dom";
import { getChains } from "../scheme";
import Page from "./Page";

export default function App() {
  const allChains = getChains();
  const chainsName = Object.keys(allChains);
  const routes = chainsName.map((name) => (
    <Route
      key={name}
      path={name}
      element={<Page currentName={name} allChains={allChains} />}
    />
  ));
  return (
    <HashRouter>
      <Routes>
        <Route
          path="/"
          element={<Page currentName={chainsName[0]} allChains={allChains} />}
        />
        {routes}
      </Routes>
    </HashRouter>
  );
}
