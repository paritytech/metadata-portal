import { Listbox } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/solid";
import { LOGOS, PORTALS } from "../constants";
import { Chains } from "../scheme";
import { capitalizeFirstLetter, cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";
import { Hr } from "./Hr";

export const NetworkAndPortalSelectMobile = ({
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
          <div className="text-sm text-neutral-400">Metadata Portal</div>
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
          <div className="text-sm text-neutral-400">Selected Network</div>
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
                  <img
                    src={LOGOS[chain as "polkadot"]}
                    className="w-8 rounded-full"
                  />
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
