import { useEffect, useState } from "react";
import { Chains } from "../scheme";
import { About } from "./About";
import "./App.css";
import { AppLinks } from "./AppLinks";
import { FAQ } from "./FAQ";
import { Hr } from "./Hr";
import { Links } from "./Links";
import { Network } from "./Network";
import { NetworkSelect } from "./NetworkSelect";

export default function App() {
  const [chains, setChains] = useState<Chains>({} as Chains);
  const [currentChain, setCurrentChain] = useState<string>("");
  const spec = chains[currentChain];

  useEffect(() => {
    fetch("data.json")
      .then((res) => res.json())
      .catch(() => {
        console.error(
          "Unable to fetch data file. Run `make collector` to generate it"
        );
      })
      .then(setChains);
  }, []);

  useEffect(() => {
    if (Object.keys(chains).length === 0 || currentChain) return;

    const locationChain = location.hash.replace("#/", "");
    const network =
      (Object.keys(chains).includes(locationChain) && locationChain) ||
      Object.keys(chains)[0];
    setCurrentChain(network);
  }, [chains]);

  useEffect(() => {
    if (currentChain) location.assign("#/" + currentChain);
  }, [currentChain]);

  if (!spec) return null;

  return (
    <div>
      <AppLinks />
      <div className="p-2 pb-4 space-y-2">
        <About />
        <NetworkSelect
          chains={chains}
          currentChain={currentChain}
          onSelect={setCurrentChain}
        />
        <div className="space-y-8">
          <Network spec={spec} />
          <FAQ />
          <Hr />
          <Links />
        </div>
      </div>
    </div>
  );
}
