import { Listbox } from "@headlessui/react";
import { icon } from "../icons";
import { Chains } from "../scheme";
import { capitalizeFirstLetter, cn } from "../utils";
import { useState } from "react";
import { SearchBar } from "./SearchBar";

export const NetworkSelect = ({
  chains,
  currentChain,
  onSelect,
}: {
  chains: Chains;
  currentChain: string;
  onSelect: (v: string) => void;
}) => {
  const chainList = Object.keys(chains);
  const [searchString, setSearchString] = useState("");

  const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchString(event.target.value);
  };

  const filteredItems = chainList.filter((item) =>
    item.toLowerCase().includes(searchString.toLowerCase())
  );
  return (
    <div className="w-full">
      <div className="text-black opacity-70 mb-4">Networks</div>
      {chainList.length > 10 && (
        <SearchBar
          searchString={searchString}
          setSearchString={setSearchString}
          onChange={handleSearch}
        />
      )}
      <Listbox value={currentChain} onChange={onSelect}>
        <Listbox.Options static className="space-y-2">
          {filteredItems.map((chain) => (
            <Listbox.Option key={chain} value={chain}>
              {({ selected }) => (
                <div
                  className={cn(
                    "flex items-center space-x-2 p-2 transition-colors rounded-full hover:bg-neutral-100",
                    selected && "bg-neutral-100",
                    chains[chain].relayChain && "pl-8",
                    selected ? "cursor-default" : "cursor-pointer"
                  )}
                >
                  <img src={icon(chain)} className="w-8 rounded-full" />
                  <div className="text-lg">
                    {capitalizeFirstLetter(chains[chain].title)}
                  </div>
                </div>
              )}
            </Listbox.Option>
          ))}
        </Listbox.Options>
      </Listbox>
    </div>
  );
};
