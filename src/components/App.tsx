import { useEffect, useState } from "react";
import { Chains, ChainSpec, QrInfo } from "../scheme";
import { useLocation } from "react-router-dom";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";
import { useLocalStorage } from "../hooks/useLocalStorage";
import { capitalizeFirstLetter } from "../utils";

import "./App.css";
import Sidebar from "./Sidebar";

export default function App() {
  const [localNetwork, setLocalNetwork] = useLocalStorage("chosenNetwork");
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [sidebarStyle, setSidebarStyle] = useState<string>("");
  const svgClass = "inline mr-2 h-7";

  const [allChains, setAllChains] = useState<Chains>({} as Chains);

  useEffect(() => {
    fetch("./data.json").then(async (res) => {
      setAllChains(await res.json());
    });
  }, []);

  // replace existing url hash in order to identify the network
  // from the url if it exists (it prioritizes over every other option below)
  const location = useLocation().hash.replace("#/", "");
  // check if URL exists in given Networks, if not
  // check localStorage if it contains a - from before - chosen network, if not
  // retrieve the 1st available network from the given ones, else (rare and wrong case)
  // default to polkadot
  const currentName =
    (Object.keys(allChains).includes(location) && location) ||
    localNetwork ||
    Object.keys(allChains)[0] ||
    "polkadot";

  const [currentNetwork, setCurrentNetwork] = useState<string>(currentName);
  const [metadataQr, setMetadataQr] = useState<QrInfo>(
    allChains[currentName]?.metadataQr
  );
  const [chain, setChain] = useState<ChainSpec>(allChains[currentName]);
  const [specsQr, setSpecsQr] = useState<QrInfo>(
    allChains[currentName]?.specsQr
  );

  useEffect(() => {
    const name = currentNetwork?.toLowerCase();
    if (name) {
      setChain(allChains[name]);
      setMetadataQr(allChains[name]?.metadataQr);
      setSpecsQr(allChains[name]?.specsQr);
      // In case the changed name is not the same as the url
      // then change the url accordingly to the selected network
      if (name !== location) window.location.assign("#/" + name);
    }
  }, [currentNetwork, allChains]);

  useEffect(() => {
    if (isOpen) {
      setSidebarStyle(
        "w-64 bg-white px-4 absolute md:left-0 md:h-[91vh] h-[92vh] md:border-r-0 border-r-2 border-neutral-200 z-30 left-0"
      );
    } else {
      setSidebarStyle(
        "w-64 bg-white px-4 absolute md:left-0 md:h-[91vh] h-[92vh] md:border-r-0 border-r-2 border-neutral-200 z-30 md:z-0 left-[-17rem]"
      );
    }
  }, [isOpen]);

  document.body.style.backgroundColor = "#F5F5F5";
  const color: string = allChains[currentName]?.color || "#9C9C9C";

  return !chain ? (
    <></>
  ) : (
    <div className="flex flex-col bg-white">
      <div
        className="md:flex justify-between px-10 py-2 items-center text-xl"
        style={{ backgroundColor: color }}
      >
        <div className="text-white md:w-1 font-bold text-2xl text-left m-auto md:m-0">
          Metadata Update Portal
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
      <div className="flex h-[91vh] md:h-auto">
        <Sidebar
          allChains={allChains}
          sidebarStyle={sidebarStyle}
          currentNetwork={currentNetwork}
          setLocalNetwork={setLocalNetwork}
          setCurrentNetwork={setCurrentNetwork}
          color={color}
        />
        {/** darker layer*/}
        {isOpen && (
          <div
            className="absolute w-full h-[91vh] bg-black opacity-80 z-20 visible"
            onClick={() => {
              setIsOpen(!isOpen);
            }}
          />
        )}
        {/** Main content */}
        <div className="m-auto flex flex-col md:absolute md:left-60 md:pl-20 md:m-0">
          <div className="md:flex flex-row flex-wrap justify-center md:pt-8">
            <div
              className="px-2 py-2 rounded-lg border-gray-600 bg-white text-black"
              style={{ minWidth: "25rem" }}
            >
              <div className="flex justify-between mx-8 py-8 border-b-2 border-gray-200 items-center">
                <h1 className="text-lg md:text-2xl" style={{ color }}>
                  Metadata #{chain.metadataVersion}
                </h1>
                <div className="flex border-2 border-black rounded-xl p-2">
                  {metadataQr.signedBy ? (
                    <div className="text-black font-normal">
                      <BadgeCheckIcon className={svgClass} />
                      Signed by {metadataQr.signedBy}
                    </div>
                  ) : (
                    <div className="text-red-500">
                      <ExclamationCircleIcon className={svgClass} />
                      Unsigned
                    </div>
                  )}
                </div>
              </div>
              <div className="px-12 justify-center pt-8">
                <QrCode path={metadataQr.path} />
                <div className="text-black py-5 w-full">
                  <Specs chainSpecs={{ ...chain }} color={color} />
                  <AddToSigner {...specsQr} color={color} />
                </div>
              </div>
            </div>
          </div>
          <div className="flex w-full p-8 justify-evenly items-center">
            <a
              href="https://parity.io/signer/"
              target="_blank"
              className="text-black underline basis-40 m-1 text-center"
              rel="noreferrer"
            >
              Parity Signer
            </a>
            <a
              href="https://www.parity.io/"
              target="_blank"
              className="text-black underline basis-40 m-1 text-center"
              rel="noreferrer"
            >
              Developed by Parity
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
