import { useEffect, useState } from "react";
import { Chains } from "../scheme";
import { useLocation } from "react-router-dom";
import { useLocalStorage } from "../hooks/useLocalStorage";
import { capitalizeFirstLetter } from "../utils";

import "./App.css";
import Sidebar from "./Sidebar";
import { AppLinks } from "./AppLinks";
import { Network } from "./Network";

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

  const color = specs.color;
  return (
    <div className="flex flex-col w-full overflow-auto">
      <AppLinks />
      <div
        className="md:hidden md:invisible px-2 text-white font-bold text-2xl flex flex-row"
        style={{ backgroundColor: color }}
      >
        <div
          className="bg-white py-2 visible items-center"
          style={{ backgroundColor: color }}
        >
          <BurgerButton isOpen={isOpen} onClick={() => setIsOpen(!isOpen)} />
        </div>
        <span className="self-center">
          {capitalizeFirstLetter(specs.title)}
        </span>
      </div>
      <div className="flex flex-row">
        <Sidebar
          allChains={allChains}
          currentNetwork={currentNetwork}
          setLocalStorageNetwork={setLocalStorageNetwork}
          setCurrentNetwork={setCurrentNetwork}
          setIsOpen={setIsOpen}
          isOpen={isOpen}
        />
        {/** darker layer*/}
        {isOpen && (
          <div
            className="absolute w-full bg-black h-full opacity-80 z-20 visible"
            onClick={() => {
              setIsOpen(!isOpen);
            }}
          />
        )}
        <main className="p-4 w-full">
          <Network spec={specs} />
        </main>
      </div>
    </div>
  );
}

interface BurgerButtonProps {
  isOpen: boolean;
  onClick: () => void;
}

function BurgerButton({ isOpen, onClick }: BurgerButtonProps) {
  return (
    <button
      className="flex top-0 left-0 relative w-8 h-10 text-white focus:outline-none"
      onClick={onClick}
    >
      <div className="absolute w-5 transform -translate-x-1/2 -translate-y-1/2 top-1/2">
        <span
          className={`absolute h-0.5 w-5 bg-white transform transition duration-200 ease-in-out ${
            isOpen ? "rotate-45 delay-100" : "-translate-y-1.5"
          }`}
        ></span>
        <span
          className={`absolute h-0.5 bg-white transform transition-all duration-100 ease-in-out ${
            isOpen ? "w-0 opacity-50" : "w-5 delay-100 opacity-100"
          }`}
        ></span>
        <span
          className={`absolute h-0.5 w-5 bg-white transform transition duration-200 ease-in-out ${
            isOpen ? "-rotate-45 delay-100" : "translate-y-1.5"
          }`}
        ></span>
      </div>
    </button>
  );
}
