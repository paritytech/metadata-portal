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
      <div className="flex flex-col lg:flex-row">
        <div className="lg:sticky lg:top-0 w-full p-2 lg:p-4 lg:pr-2 lg:pt-24 lg:max-w-xs lg:h-screen">
          <div className="lg:hidden mb-2">
            <About />
          </div>
          <div className="lg:hidden">
            <NetworkAndPortalSelectMobile
              chains={chains}
              currentChain={currentChain}
              onSelect={setCurrentChain}
            />
          </div>
          <div className="hidden lg:block mb-4">
            <PortalSelect />
          </div>
          <div className="hidden lg:block mb-4">
            <About />
          </div>
          <div className="hidden lg:block">
            <NetworkSelect
              chains={chains}
              currentChain={currentChain}
              onSelect={setCurrentChain}
            />
          </div>
        </div>
        <div className="w-full p-2 pt-0 pb-8 lg:p-4 lg:pl-2 lg:pt-24 space-y-4">
          <Network spec={spec} />
          <FAQ />
          <div className="lg:hidden">
            <Hr />
          </div>
          <div className="lg:hidden">
            <Links />
          </div>
        </div>
      </div>
    </div>
  );
}
