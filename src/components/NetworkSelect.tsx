import { Listbox } from "@headlessui/react";
import { ChevronDownIcon, XMarkIcon } from "@heroicons/react/24/solid";
import { Chains } from "../scheme";
import { capitalizeFirstLetter, cn } from "../utils";
import "./App.css";
import { ChevronIcon } from "./ChevronIcon";
import { Hr } from "./Hr";

const PORTALS = ["Parity Portal"];

export const NetworkSelect = ({
  chains,
  currentChain,
  onSelect,
}: {
  chains: Chains;
  currentChain: string;
  onSelect: (v: string) => void;
}) => (
  <div className="border border-neutral-200 p-4 rounded-4xl space-y-3">
    <div>
      <Listbox value={PORTALS[0]}>
        <Listbox.Button className="block w-full text-left">
          <div className="text-sm text-neutral-500">Metadata Portal</div>
          <div className="flex items-center justify-between w-full text-lg">
            <span>{PORTALS[0]}</span>
            <ChevronIcon />
          </div>
        </Listbox.Button>
        <Listbox.Options className="fixed inset-0 p-4 space-y-1 z-10 min-h-screen overflow-y-scroll bg-white">
          <div className="flex items-center justify-between text-2xl mb-4">
            <div>Metadata Portal</div>
            <Listbox.Button>
              <XMarkIcon className="w-6 h-6" />
            </Listbox.Button>
          </div>
          {PORTALS.map((portal) => (
            <Listbox.Option key={portal} value={portal}>
              {({ selected }) => (
                <div
                  className={cn(
                    "flex items-center space-x-2 px-2 py-1",
                    selected && "bg-neutral-100 rounded-full"
                  )}
                >
                  <div className="text-xl">{portal}</div>
                </div>
              )}
            </Listbox.Option>
          ))}
        </Listbox.Options>
      </Listbox>
    </div>
    <Hr />
    <div>
      <Listbox value={currentChain} onChange={onSelect}>
        <Listbox.Button className="block w-full text-left">
          <div className="text-sm text-neutral-500">Selected Network</div>
          <div className="flex items-center justify-between w-full text-lg">
            <span>{capitalizeFirstLetter(chains[currentChain]?.title)}</span>
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
                    selected && "bg-neutral-100 rounded-full"
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
  </div>
);
