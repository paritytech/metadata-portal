import { Listbox } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/solid";
import { icon } from "../icons";
import { Chains } from "../scheme";
import { formatTitle, cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";

export const NetworkSelectMobile = ({
  chains,
  currentChain,
  onSelect,
}: {
  chains: Chains;
  currentChain: string;
  onSelect: (v: string) => void;
}) => {
  return (
    <div className="border border-neutral-200 p-4 rounded-4xl space-y-3">
      <div>
        <Listbox value={currentChain} onChange={onSelect}>
          <Listbox.Button className="block w-full text-left">
            <div className="text-sm text-black opacity-70">
              Selected Network
            </div>
            <div className="flex items-center justify-between w-full text-lg">
              <span>{formatTitle(chains[currentChain]?.title)}</span>
              <ChevronIcon />
            </div>
          </Listbox.Button>
          <Listbox.Options className="fixed inset-0 p-4 space-y-1 z-10 min-h-screen overflow-y-scroll bg-white">
            <div className="flex items-center justify-between text-2xl mb-4">
              <div>Selected Network</div>
              <Listbox.Button>
                <XMarkIcon className="w-6 h-6" />
              </Listbox.Button>
            </div>
            {Object.keys(chains).map((chain) => (
              <Listbox.Option key={chain} value={chain}>
                {({ selected }) => (
                  <div
                    className={cn(
                      "flex items-center space-x-2 px-2 py-1",
                      selected && "bg-neutral-100 rounded-full",
                    )}
                  >
                    <img src={icon(chain)} className="w-8 rounded-full" />
                    <div className="text-xl">
                      {formatTitle(chains[chain].title)}
                    </div>
                  </div>
                )}
              </Listbox.Option>
            ))}
          </Listbox.Options>
        </Listbox>
      </div>
    </div>
  );
};
