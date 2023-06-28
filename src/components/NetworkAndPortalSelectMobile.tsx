import { Listbox } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/solid";
import { icon } from "../icons";
import { Chains, Portals } from "../scheme";
import { capitalizeFirstLetter, cn, currentPortalKey } from "../utils";
import { ChevronIcon } from "./ChevronIcon";
import { Hr } from "./Hr";

export const NetworkAndPortalSelectMobile = ({
  chains,
  portals,
  currentChain,
  onSelect,
}: {
  chains: Chains;
  portals: Portals;
  currentChain: string;
  onSelect: (v: string) => void;
}) => {
  const current = currentPortalKey(portals);
  const hasManyPortals = Boolean(current) && Object.keys(portals).length > 1;

  return (
    <div className="border border-neutral-200 p-4 rounded-4xl space-y-3">
      {hasManyPortals && (
        <>
          <div>
            <Listbox value={current}>
              <Listbox.Button className="block w-full text-left">
                <div className="text-sm text-black opacity-70">
                  Metadata Portal
                </div>
                <div className="flex items-center justify-between w-full text-lg">
                  <span>{portals[current].name}</span>
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
                {Object.keys(portals).map((portal) => (
                  <Listbox.Option key={portal} value={portal}>
                    {({ selected }) =>
                      selected ? (
                        <div
                          className={cn(
                            "flex items-center space-x-2 px-2 py-1",
                            selected && "bg-neutral-100 rounded-full"
                          )}
                        >
                          <div className="text-xl">{portals[portal].name}</div>
                        </div>
                      ) : (
                        <a
                          className={cn(
                            "flex items-center space-x-2 px-2 py-1",
                            selected && "bg-neutral-100 rounded-full"
                          )}
                          href={portals[portal].url}
                        >
                          <div className="text-xl">{portals[portal].name}</div>
                        </a>
                      )
                    }
                  </Listbox.Option>
                ))}
              </Listbox.Options>
            </Listbox>
          </div>
          <Hr />
        </>
      )}
      <div>
        <Listbox value={currentChain} onChange={onSelect}>
          <Listbox.Button className="block w-full text-left">
            <div className="text-sm text-black opacity-70">
              Selected Network
            </div>
            <div className="flex items-center justify-between w-full text-lg">
              <span>
                {currentChain === "node-subtensor"
                  ? "Bittensor"
                  : capitalizeFirstLetter(chains[currentChain]?.title)}
              </span>
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
                    <img src={icon(chain)} className="w-8 rounded-full" />
                    <div className="text-xl">
                      {chain === "node-subtensor"
                        ? "Bittensor"
                        : capitalizeFirstLetter(chains[chain].title)}
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
