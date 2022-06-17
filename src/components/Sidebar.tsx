import React, { Dispatch, SetStateAction, useState } from "react";
import { Chains } from "../scheme";
import { capitalizeFirstLetter } from "../utils";

interface Props {
  allChains: Chains;
  sidebarStyle: string;
  currentNetwork: string;
  setCurrentNetwork: Dispatch<SetStateAction<string>>;
  setLocalStorageNetwork: Dispatch<SetStateAction<string>>;
  color: string;
}

const searchStringInArray = (str: string, strArray: string[]) => {
  const a = [];
  for (let j = 0; j < strArray.length; j++) {
    if (strArray[j].match(str)) a.push(strArray[j]);
  }
  return a;
};

export default function Sidebar({
  allChains,
  sidebarStyle,
  currentNetwork,
  setCurrentNetwork,
  setLocalStorageNetwork,
  color,
}: Props): JSX.Element {
  const [searchResults, setSearchResults] = useState<string[]>(
    Object.keys(allChains)
  );

  return (
    <div className={sidebarStyle}>
      {/** Search Bar */}
      <div className="flex justify-center pt-6 h-[7vh]">
        <div className="mb-3 xl:w-96">
          <div className="input-group relative flex flex-wrap items-stretch w-full mb-4">
            <input
              type="search"
              className="form-control relative flex-auto min-w-0 block w-full px-3 py-1.5 text-base font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
              placeholder="Search"
              onChange={(a) => {
                setSearchResults(
                  searchStringInArray(a.target.value, Object.keys(allChains))
                );
              }}
            />
          </div>
        </div>
      </div>
      {/** SEARCH BAR END */}
      <ul className="relative overflow-auto h-[62vh]">
        {searchResults.map((c) => (
          <li className="relative" key={c}>
            {currentNetwork === c ? (
              <div
                className="flex items-center text-sm py-4 my-6 px-3 h-12 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded"
                style={{ backgroundColor: color }}
              >
                <div className="flex items-center text-xl text-white font-inter">
                  <div className="network_icon">{c}</div>
                  <div className="network_name">{capitalizeFirstLetter(c)}</div>
                </div>
              </div>
            ) : (
              <div
                className="flex items-center text-sm py-4 my-6 px-2 mx-2 h-12 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded hover:text-gray-900 hover:bg-gray-100 transition duration-300 ease-in-out hover:cursor-pointer"
                onClick={() => {
                  setCurrentNetwork(c);
                  setLocalStorageNetwork(c);
                }}
              >
                <div className="flex items-center text-xl font-inter">
                  <div className="network_icon text-[#9E9E9E]">{c}</div>
                  <div className="network_name">{capitalizeFirstLetter(c)}</div>
                </div>
              </div>
            )}
          </li>
        ))}
      </ul>
      <div className="bottom-4 w-50 text-md h-[17vh]">
        <div className="text-left mr-6 pt-6 border-t border-neutral-300">
          Metadata Portal is a self-hosted web page which shows you the latest
          metadata for a given network.
        </div>
        <a
          className="text-left mt-0 inline-block md:w-fit w-full"
          href="https://github.com/paritytech/metadata-portal"
          target={"blank"}
        >
          <div className="flex float-left hover:cursor-pointer hover:text-gray-500 font-bold">
            More on GitHub
          </div>
        </a>
        <a href="https://www.parity.io/terms/" target="_blank" rel="noreferrer">
          <div className="text-left hover:text-gray-500 font-bold">
            Terms & Services
          </div>
        </a>
      </div>
    </div>
  );
}
