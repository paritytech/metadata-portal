import { useEffect, useState } from "react";
import { Chains, ChainSpec, QrInfo } from "../scheme";
import { useLocation } from "react-router-dom";
import { useLocalStorage } from "../hooks/useLocalStorage";
import { capitalizeFirstLetter } from "../utils";

import "./App.css";
import Sidebar from "./Sidebar";
import Main from "./Main";

export default function App() {
  const [localStorageNetwork, setLocalStorageNetwork] =
    useLocalStorage("chosenNetwork");
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [sidebarStyle, setSidebarStyle] = useState<string>("");

  const [allChains, setAllChains] = useState<Chains>({} as Chains);

  useEffect(() => {
    const fetchData = async () => {
      const data = await fetch("data.json")
          .then(response => response.json())
          .catch(e => {
            console.error("Unable to fetch data file. Run `make collector` to generate it")
            return e;
          });
      return await data as Chains;
    };
    fetchData().then(r => {
      setAllChains(r)
    });
  }, []);

  // replace existing url hash in order to identify the network
  // from the url if it exists (it prioritizes over every other option below)
  const location = useLocation().hash.replace("#/", "");
  // check if URL exists in given Networks, if not
  // check localStorage if it contains a - from before - chosen network, if not
  // retrieve the 1st available network from the given ones, else (rare and wrong case)
  // default to polkadot
  const [currentNetwork, setCurrentNetwork] = useState<string>(
    (Object.keys(allChains).includes(location) && location) ||
      localStorageNetwork && localStorageNetwork.toLowerCase() ||
      Object.keys(allChains)[0] ||
      "acala"
  );

  const [chain, setChain] = useState<ChainSpec>(allChains[currentNetwork]);
  const [specsQr, setSpecsQr] = useState<QrInfo>(
    allChains[currentNetwork]?.specsQr
  );
  const specs = allChains[currentNetwork];

  useEffect(() => {
    const name = currentNetwork?.toLowerCase();
    if (name) {
      setChain(allChains[name]);
      setSpecsQr(allChains[name]?.specsQr);
      // In case the changed name is not the same as the url
      // then change the url accordingly to the selected network
      if (name !== location) window.location.assign("#/" + name);
    }
  }, [currentNetwork, allChains]);

  useEffect(() => {
    if (isOpen) {
      setSidebarStyle(
        "w-64 bg-white px-6 absolute md:left-0 md:h-[89vh] h-[89vh] md:border-r-0 border-r-2 border-neutral-200 z-30 left-0"
      );
    } else {
      setSidebarStyle(
        "w-64 bg-white px-6 absolute md:left-0 md:h-[89vh] h-[89vh] md:border-r-0 border-r-2 border-neutral-200 z-30 md:z-0 left-[-17rem]"
      );
    }
  }, [isOpen]);

  document.body.style.backgroundColor = "#F5F5F5";
  const { color } = allChains[currentNetwork] || { color: "#9C9C9C" };
  const qr = allChains[currentNetwork]?.metadataQr;

  if (!specs) {
    return null;
  }

  return !chain ? (
    <></>
  ) : (
    <div className="flex flex-col bg-white">
      <div
        className="md:flex justify-between px-10 py-2 items-center text-xl"
        style={{ backgroundColor: color }}
      >
        <div className="text-white font-bold text-2xl text-left m-auto flex flex-row md:flex-col md:m-0">
          <div>Metadata</div>
          <div className="md:pl-0 pl-2">Update Portal</div>
        </div>
        <div
          className="bg-white py-2 visible md:invisible md:hidden flex text-white items-center"
          style={{ backgroundColor: color }}
        >
          <button
            className="md:hidden flex top-0 left-0 relative w-8 h-10 text-white focus:outline-none"
            onClick={() => setIsOpen(!isOpen)}
          >
            <div className="absolute w-5 transform -translate-x-1/2 -translate-y-1/2 top-1/2">
              <span
                className={`absolute h-0.5 w-5 bg-white transform transition duration-300 ease-in-out ${
                  isOpen ? "rotate-45 delay-200" : "-translate-y-1.5"
                }`}
              ></span>
              <span
                className={`absolute h-0.5 bg-white transform transition-all duration-200 ease-in-out ${
                  isOpen ? "w-0 opacity-50" : "w-5 delay-200 opacity-100"
                }`}
              ></span>
              <span
                className={`absolute h-0.5 w-5 bg-white transform transition duration-300 ease-in-out ${
                  isOpen ? "-rotate-45 delay-200" : "translate-y-1.5"
                }`}
              ></span>
            </div>
          </button>
          {capitalizeFirstLetter(chain.name)}
        </div>
      </div>
      <div className="flex h-[89vh] md:h-auto">
        <Sidebar
          allChains={allChains}
          sidebarStyle={sidebarStyle}
          currentNetwork={currentNetwork}
          setLocalStorageNetwork={setLocalStorageNetwork}
          setCurrentNetwork={setCurrentNetwork}
          color={color}
        />
        {/** darker layer*/}
        {isOpen && (
          <div
            className="absolute w-full h-[89vh] bg-black opacity-80 z-20 visible"
            onClick={() => {
              setIsOpen(!isOpen);
            }}
          />
        )}
        <Main
          metadataQr={qr}
          specsQr={specsQr}
          color={color}
          chain={chain}
          specs={specs}
        />
      </div>
    </div>
  );
}
