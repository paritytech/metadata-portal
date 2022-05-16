import { useEffect, useState } from "react";
import { ChainSpec, getChains, QrInfo } from "../scheme";
import { useLocation } from "react-router-dom";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";
import { useLocalStorage } from "../hooks/useLocalStorage";

import "./App.css";

const searchStringInArray = (str: string, strArray: string[]) => {
  const a = [];
  for (let j = 0; j < strArray.length; j++) {
    if (strArray[j].match(str)) a.push(strArray[j]);
  }
  return a;
};

export default function App() {
  const [localNetwork, setLocalNetwork] = useLocalStorage("chosenNetwork");

  const allChains = getChains();
  // replace existing url hash in order to identify the network
  // from the url if it exists (it prioritarizes over every other option below)
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
  const svgClass = "inline mr-2 h-7";

  const [currentNetwork, setCurrentNetwork] = useState<string>(currentName);
  const [metadataQr, setMetadataQr] = useState<QrInfo>(
    allChains[currentName].metadataQr
  );

  const [specsQr, setSpecsQr] = useState<QrInfo>(
    allChains[currentName].specsQr
  );

  const [chain, setChain] = useState<ChainSpec>(allChains[currentName]);

  const [searchResults, setSearchResults] = useState<string[]>(
    Object.keys(allChains)
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
  }, [currentNetwork]);

  document.body.style.backgroundColor = "#F5F5F5";
  const { color } = allChains[currentName];

  return (
    <div className="flex flex-col">
      <div
        className="lg:flex justify-between px-10 py-2 items-center h-28"
        style={{ backgroundColor: color }}
      >
        <div className="text-white lg:w-1 font-bold text-2xl lg:text-left text-center">
          Metadata Update Portal
        </div>
        <div className="text-white font-bold">
          <a
            href="https://www.parity.io/terms/"
            target="_blank"
            className="text-white underline basis-40 m-1 text-center"
            rel="noreferrer"
          >
            Terms of Service
          </a>
        </div>
      </div>
      <div className="flex">
        <div
          className="w-64 bg-white px-4 absolute"
          style={{ height: "calc(100vh - 7rem)" }}
        >
          {/** SEARCH BAR */}
          <div className="flex justify-center pt-6">
            <div className="mb-3 xl:w-96">
              <div className="input-group relative flex flex-wrap items-stretch w-full mb-4">
                <input
                  type="search"
                  className="form-control relative flex-auto min-w-0 block w-full px-3 py-1.5 text-base font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
                  placeholder="Search"
                  onChange={(a) => {
                    setSearchResults(
                      searchStringInArray(
                        a.target.value,
                        Object.keys(allChains)
                      )
                    );
                  }}
                />
              </div>
            </div>
          </div>
          {/** SEARCH BAR END */}
          <ul className="relative">
            {searchResults.map((c) => (
              <li className="relative" key={c}>
                {currentNetwork === c ? (
                  <div
                    className="flex items-center text-sm py-4 my-6 px-6 h-12 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded"
                    style={{ backgroundColor: color }}
                  >
                    <div className="flex items-center text-xl text-white">
                      <div className="network_icon">{c}</div>
                      <div>{c.charAt(0).toUpperCase() + c.slice(1)}</div>
                    </div>
                  </div>
                ) : (
                  <div
                    className="flex items-center text-sm py-4 my-6 px-4 mx-2 h-12 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded hover:text-gray-900 hover:bg-gray-100 transition duration-300 ease-in-out hover:cursor-pointer"
                    onClick={() => {
                      setCurrentNetwork(c);
                      setLocalNetwork(c);
                    }}
                  >
                    <div className="flex items-center text-xl text-black">
                      <div className="network_icon">{c}</div>
                      <div>{c.charAt(0).toUpperCase() + c.slice(1)}</div>
                    </div>
                  </div>
                )}
              </li>
            ))}
          </ul>
          <div className="bottom-4 fixed w-60 text-md">
            <div className="text-left mb-6 mr-4 pt-6 border-t	border-neutral-300">
              Metadata Portal is a self-hosted web page which shows you the
              latest metadata for a given network.
            </div>
            <a
              className="lg:text-left text-center lg:mt-0 mt-5 lg:block inline-block lg:w-fit w-full"
              href="https://github.com/paritytech/metadata-portal"
              target={"blank"}
            >
              <div className="flex float-left hover:cursor-pointer hover:text-gray-500">
                More on GitHub
              </div>
            </a>
          </div>
        </div>
        <div className="flex flex-col absolute left-60 pl-20">
          <div className="md:flex flex-row flex-wrap justify-center lg:pt-8">
            <div
              className="px-2 py-2 rounded-lg border-gray-600 bg-white text-black"
              style={{ minWidth: "35rem" }}
            >
              <div className="flex justify-between mx-8 py-8 border-b-2 border-gray-200 ">
                <h1 className="text-lg lg:text-2xl" style={{ color }}>
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
