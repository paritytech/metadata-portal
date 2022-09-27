import { useEffect, useState } from "react";
import { Chains } from "../scheme";
import { useLocation } from "react-router-dom";
import { useLocalStorage } from "../hooks/useLocalStorage";
import { Tab } from "@headlessui/react";
import { capitalizeFirstLetter, getBackgroundStyle } from "../utils";

import "./App.css";
import Sidebar from "./Sidebar";
import MetadataTab from "./MetadataTab";
import SpecsTab from "./SpecsTab";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function App() {
  const [localStorageNetwork, setLocalStorageNetwork] =
    useLocalStorage("chosenNetwork");
  const [isOpen, setIsOpen] = useState<boolean>(false);

  const [allChains, setAllChains] = useState<Chains>({} as Chains);
  let dataFileName = "data.json";
  if (useLocation().pathname.split("/").indexOf("dev") > 0) {
    dataFileName = "data_dev.json";
  }
  useEffect(() => {
    const fetchData = async () => {
      const data = await fetch(dataFileName)
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
      <div
        className="md:hidden md:invisible px-2 text-white font-bold text-2xl flex flex-row"
        style={getBackgroundStyle(color)}
      >
        <div
          className="bg-white py-2 visible items-center"
          style={getBackgroundStyle(color)}
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
        <Tab.Group>
          <div className="flex flex-col w-full px-2 md:px-8">
            <Tab.List className="flex flex-row w-full border-b border-neutral-300">
              {["Metadata", "Chain Specs"].map((title) => (
                <Tab
                  key={title}
                  className={({ selected }) =>
                    classNames(
                      "w-32 h-12 py-2.5 font-semibold leading-5 mb-[-1px] focus-visible:outline-none",
                      selected ? `border-b-2` : "!text-black"
                    )
                  }
                  style={{ borderColor: `${color}`, color: `${color}` }}
                >
                  {title}
                </Tab>
              ))}
            </Tab.List>
            <Tab.Panels>
              <Tab.Panel className="flex justify-center">
                <MetadataTab specs={{ ...specs }} key={specs.title} />
              </Tab.Panel>
              <Tab.Panel>
                <SpecsTab specs={{ ...specs }} />
              </Tab.Panel>
            </Tab.Panels>
          </div>
        </Tab.Group>
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
