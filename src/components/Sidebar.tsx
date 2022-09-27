import { Dispatch, SetStateAction, useState } from "react";
import { Chains } from "../scheme";
import { capitalizeFirstLetter, getBackgroundStyle } from "../utils";
import { ArrowSmRightIcon } from "@heroicons/react/solid";

interface Props {
  allChains: Chains;
  currentNetwork: string;
  setCurrentNetwork: Dispatch<SetStateAction<string>>;
  setLocalStorageNetwork: Dispatch<SetStateAction<string>>;
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
}

const searchStringInArray = (str: string, allChains: Chains) => {
  const result: string[] = [];

  Object.keys(allChains).forEach((key) => {
    if (allChains[key].title.toUpperCase().match(str.toUpperCase()))
      result.push(key);
  });
  result.sort((a, b) => {
    if (
      (allChains[a].testnet && allChains[b].testnet) ||
      (!allChains[a].testnet && !allChains[b].testnet)
    ) {
      return allChains[a].title.localeCompare(allChains[b].title);
    } else if (allChains[a].testnet) {
      return 1;
    } else {
      return -1;
    }
  });
  return result;
};

export default function Sidebar({
  allChains,
  currentNetwork,
  setCurrentNetwork,
  setLocalStorageNetwork,
  isOpen,
  setIsOpen,
}: Props): JSX.Element {
  const [searchResults, setSearchResults] = useState<string[]>(
    Object.keys(allChains).sort((a, b) => {
      if (
        (allChains[a].testnet && allChains[b].testnet) ||
        (!allChains[a].testnet && !allChains[b].testnet)
      ) {
        return allChains[a].title.localeCompare(allChains[b].title);
      } else if (allChains[a].testnet) {
        return 1;
      } else {
        return -1;
      }
    })
  );

  const specs = allChains[currentNetwork];

  return (
    <div
      className={`flex-col h-auto min-h-screen w-80 min-w-[20rem] bg-neutral-100 px-6 justify-between absolute ${
        isOpen ? "flex" : "hidden"
      } z-30 md:relative md:flex`}
    >
      <div className=" ">
        <h1 className="font-bold text-xl my-6">
          Metadata & Chain Spec Update Portal
        </h1>

        {/** Search Bar */}
        <div className="flex ">
          <div className="w-full">
            <div className="input-group relative flex flex-wrap items-stretch mb-4">
              <span className="text-gray-500">Network</span>
              <input
                type="search"
                className="form-control relative flex-auto min-w-0 block w-full px-3 py-1.5 text-base font-normal  bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
                placeholder="Search"
                onChange={(a) => {
                  setSearchResults(
                    searchStringInArray(a.target.value, allChains)
                  );
                }}
              />
            </div>
          </div>
        </div>
        {/** SEARCH BAR END */}
        <ul className="relative overflow-auto">
          {searchResults.map((c) => (
            <li className="relative" key={c}>
              {currentNetwork === c ? (
                <div
                  className="flex items-center text-sm p-2 mb-3 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded"
                  style={getBackgroundStyle(specs.color)}
                >
                  <div className="flex items-center text-xl text-white font-inter">
                    <div className="network_icon">
                      <img src={allChains[c].icon} />
                    </div>
                    <div className="network_name">
                      {capitalizeFirstLetter(allChains[c].title)}
                    </div>
                  </div>
                </div>
              ) : (
                <div
                  className="flex items-center text-sm p-2 mb-3 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded hover:text-gray-900 hover:bg-gray-100 transition duration-300 ease-in-out hover:cursor-pointer"
                  onClick={() => {
                    setCurrentNetwork(c);
                    setLocalStorageNetwork(c);
                    setIsOpen(false);
                  }}
                >
                  <div className="flex items-center text-xl font-inter">
                    <div className="network_icon">
                      <img src={allChains[c].icon} />
                    </div>
                    <div className="network_name">
                      {capitalizeFirstLetter(allChains[c].title)}
                    </div>
                  </div>
                </div>
              )}
            </li>
          ))}
        </ul>
      </div>
      <div className="text-md">
        <div className="text-left mr-6 pt-6 border-t border-neutral-300">
          Metadata Portal is a self-hosted web page which shows you the latest
          metadata for a given network.
        </div>
        <a
          className="text-left mt-3 inline-block md:w-fit w-full"
          href="https://github.com/nova-wallet/metadata-portal"
          target={"blank"}
        >
          <div className="flex float-left font-bold">
            More on GitHub <ArrowSmRightIcon className="inline ml-1 w-4" />
          </div>
        </a>
        <a
          href="https://novawallet.io/terms"
          className="inline-block mt-1 mb-4"
          target="_blank"
          rel="noreferrer"
        >
          <div className="text-left font-bold">
            Terms & Services <ArrowSmRightIcon className="inline ml-1 w-4" />
          </div>
        </a>
      </div>
    </div>
  );
}
