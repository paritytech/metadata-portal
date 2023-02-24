import { Listbox } from "@headlessui/react";
import { LOGOS } from "../constants";
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
    <div className="text-neutral-400 mb-4">Networks</div>
    <Listbox value={currentChain} onChange={onSelect}>
      <Listbox.Options static className="space-y-2">
        {Object.keys(chains).map((chain) => (
          <Listbox.Option key={chain} value={chain}>
            {({ selected }) => (
              <div
                className={cn(
                  "flex items-center space-x-2 p-2 transition-colors rounded-full hover:bg-neutral-100",
                  selected && "bg-neutral-100",
                  selected ? "cursor-default" : "cursor-pointer"
                )}
              >
                <img
                  src={LOGOS[chain as "polkadot"]}
                  className="w-8 rounded-full"
                />
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
