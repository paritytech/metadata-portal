import React from "react";
import {
    Routes,
    Route,
    HashRouter
} from "react-router-dom";
import {getChains} from "../scheme";
import InfoPage from "./InfoPage";
// @ts-ignore
import GitHubForkRibbon from 'react-github-fork-ribbon';

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
        <GitHubForkRibbon href="https://github.com/paritytech/metadata-portal" target="_blank" color="black" position="right-bottom">
          Fork me on GitHub
      </GitHubForkRibbon>
      </HashRouter>
  );
}

