import { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { useLocalStorage } from "../hooks/useLocalStorage";
import { Chains } from "../scheme";
import "./App.css";
import { AppLinks } from "./AppLinks";
import { Network } from "./Network";

const About = () => (
  <div className="text-xs bg-neutral-100 p-4 rounded-2xl">
    Metadata Portal is a self-hosted web page which shows you the latest
    metadata for a given network.
  </div>
);

const NetworkSelect = () => (
  <div className="border border-neutral-200 p-4 rounded-2xl space-y-3">
    <div>
      <div className="text-sm text-neutral-500">Metadata Portal</div>
      <div className="text-lg">Parity Portal</div>
    </div>
    <hr className="bg-neutral-200" />
    <div>
      <div className="text-sm text-neutral-500">Selected Network</div>
      <div className="text-lg">Polkadot</div>
    </div>
  </div>
);

export default function App() {
  const [localStorageNetwork, setLocalStorageNetwork] =
    useLocalStorage("chosenNetwork");
  const [isOpen, setIsOpen] = useState<boolean>(false);

  const [allChains, setAllChains] = useState<Chains>({} as Chains);

  useEffect(() => {
    const fetchData = async () => {
      const data = await fetch("data.json")
        .then((response) => response.json())
        .catch((e) => {
          console.error(
            "Unable to fetch data file. Run `make collector` to generate it"
          );
          return e;
        });
      return (await data) as Chains;
    };
    fetchData().then((r) => {
      setAllChains(r);
      const lastVisited =
        localStorageNetwork && localStorageNetwork.toLowerCase();
      const network =
        (Object.keys(r).includes(location) && location) ||
        (Object.keys(r).includes(lastVisited) && lastVisited) ||
        Object.keys(r)[0];
      setCurrentNetwork(network);
    });
  }, []);

  // replace existing url hash in order to identify the network
  // from the url if it exists (it prioritizes over every other option below)
  const location = useLocation().hash.replace("#/", "");
  // check if URL exists in given Networks, if not
  // check localStorage if it contains a - from before - chosen network, if not
  // retrieve the 1st available network from the given ones, else (rare and wrong case)
  const [currentNetwork, setCurrentNetwork] = useState<string>("");

  const specs = allChains[currentNetwork];

  useEffect(() => {
    const name = currentNetwork?.toLowerCase();
    if (name) {
      // In case the changed name is not the same as the url
      // then change the url accordingly to the selected network
      if (name !== location) window.location.assign("#/" + name);
    }
  }, [currentNetwork, allChains]);

  if (!specs) {
    return null;
  }

  return (
    <div>
      <AppLinks />
      <div className="p-2 space-y-2">
        <About />
        <NetworkSelect />
        <Network spec={specs} />
      </div>
    </div>
  );
}
