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
      <div className="flex flex-col md:flex-row">
        <div className="md:sticky md:top-0 w-full p-2 md:p-4 md:pr-2 md:pt-24 md:max-w-sm md:h-screen">
          <div className="md:hidden mb-2">
            <About />
          </div>
          <div className="md:hidden">
            <NetworkAndPortalSelectMobile
              chains={chains}
              currentChain={currentChain}
              onSelect={setCurrentChain}
            />
          </div>
          <div className="hidden md:block mt-2 mb-11">
            <PortalSelect />
          </div>
          <div className="hidden md:block mb-6">
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
        <div className="w-full p-2 pt-0 pb-8 md:p-4 md:pl-2 md:pt-24 space-y-4">
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
  );
}
