import { Listbox } from "@headlessui/react";
import { Chains } from "../scheme";
import { capitalizeFirstLetter, cn } from "../utils";

export const NetworkSelect = ({
  chains,
  currentChain,
  onSelect,
}: {
  chains: Chains;
  currentChain: string;
  onSelect: (v: string) => void;
}) => (
  <div>
    <div className="text-sm text-neutral-500 mb-2">Networks</div>
    <Listbox value={currentChain} onChange={onSelect}>
      <Listbox.Options static>
        {Object.keys(chains).map((chain) => (
          <Listbox.Option key={chain} value={chain}>
            {({ selected }) => (
              <div
                className={cn(
                  "flex items-center space-x-2 px-2 py-2",
                  selected && "bg-neutral-100 rounded-full",
                  selected ? "cursor-default" : "cursor-pointer"
                )}
              >
                <div className="web3-regular text-2xl text-center w-8">
                  {chain}
                </div>
                <div className="text-xl">
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
