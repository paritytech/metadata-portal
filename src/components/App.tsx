import { useEffect, useState } from "react";
import { Chains } from "../scheme";
import { About } from "./About";
import { AppLinks } from "./AppLinks";
import { FAQ } from "./FAQ";
import { Hr } from "./Hr";
import { Links } from "./Links";
import { Network } from "./Network";
import { NetworkAndPortalSelectMobile } from "./NetworkAndPortalSelectMobile";
import { NetworkSelect } from "./NetworkSelect";
import { PortalSelect } from "./PortalSelect";

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
      <div className="flex flex-col md:flex-row p-2 md:p-4 pb-4 md:space-x-4 space-y-2 md:space-y-0">
        <div className="md:max-w-xs space-y-2">
          <div className="md:hidden">
            <About />
          </div>
          <div className="md:hidden">
            <NetworkAndPortalSelectMobile
              chains={chains}
              currentChain={currentChain}
              onSelect={setCurrentChain}
            />
          </div>
          <div className="hidden md:block">
            <PortalSelect />
          </div>
          <div className="hidden md:block">
            <About />
          </div>
          <div className="hidden md:block">
            <NetworkSelect
              chains={chains}
              currentChain={currentChain}
              onSelect={setCurrentChain}
            />
          </div>
        </div>
        <div className="w-full">
          <div className="space-y-8">
            <Network spec={spec} />
            <FAQ />
            <div className="md:hidden">
              <Hr />
            </div>
            <div className="md:hidden">
              <Links />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
