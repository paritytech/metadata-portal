import { Dispatch, SetStateAction } from "react";
import { Chains } from "../scheme";
import { capitalizeFirstLetter } from "../utils";

export default function Sidebar({
  allChains,
  currentNetwork,
  setCurrentNetwork,
  setLocalStorageNetwork,
}: {
  allChains: Chains;
  currentNetwork: string;
  setCurrentNetwork: Dispatch<SetStateAction<string>>;
  setLocalStorageNetwork: Dispatch<SetStateAction<string>>;
}) {
  const specs = allChains[currentNetwork];

  return (
    <div className="flex flex-col w-80 p-4 z-30">
      <div>
        <ul className="relative overflow-auto">
          {Object.keys(allChains).map((chain) => (
            <li className="relative" key={chain}>
              {currentNetwork === chain ? (
                <div
                  className="flex items-center text-sm p-2 mb-3 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded"
                  style={{ backgroundColor: specs.color }}
                >
                  <div className="flex items-center text-xl text-white font-inter">
                    <div className="network_icon">{chain}</div>
                    <div className="network_name">
                      {capitalizeFirstLetter(allChains[chain].title)}
                    </div>
                  </div>
                </div>
              ) : (
                <div
                  className="flex items-center text-sm p-2 mb-3 overflow-hidden text-gray-700 text-ellipsis whitespace-nowrap rounded hover:text-gray-900 hover:bg-gray-100 transition duration-300 ease-in-out hover:cursor-pointer"
                  onClick={() => {
                    setCurrentNetwork(chain);
                    setLocalStorageNetwork(chain);
                  }}
                >
                  <div className="flex items-center text-xl font-inter">
                    <div className="network_icon text-[#9E9E9E]">{chain}</div>
                    <div className="network_name">
                      {capitalizeFirstLetter(allChains[chain].title)}
                    </div>
                  </div>
                </div>
              )}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
