import React from "react";
import {
    Routes,
    Route,
    HashRouter
} from "react-router-dom";
import {getChains} from "../data";
import InfoPage from "./InfoPage";

export default function App() {
    const allChains = getChains()
    const chainsName = Object.keys(allChains)
    const routes = chainsName.map(name => <Route key={name} path={name} element={<InfoPage currentName={name} allChains={allChains}/>} />)
  return (
      <HashRouter>
        <Routes>
            <Route path="/" element={<InfoPage currentName={chainsName[0]} allChains={allChains}/>} />
            {routes}
        </Routes>
      </HashRouter>
  );
}

