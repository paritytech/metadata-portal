import { Listbox, Transition } from "@headlessui/react";
import { Fragment } from "react";
import { Portals } from "../scheme";
import { cn, currentPortalKey } from "../utils";
import { ChevronIcon } from "./ChevronIcon";

export const PortalSelect = ({ portals }: { portals: Portals }) => {
  const current = currentPortalKey(portals);
  const hasManyPortals = Boolean(current) && Object.keys(portals).length > 1;

  if (!hasManyPortals) return null;

  return (
    <div>
      <Listbox as="div" value={current} className="relative">
        <Listbox.Button
          className={({ open }) =>
            cn(
              "w-full bordered-action py-3 hover:bg-neutral-100 transition-colors",
              open && "bg-neutral-100"
            )
          }
        >
          <div className="flex items-center justify-between -my-px">
            <div className="space-x-1">
              <span className="nowrap text-black opacity-50">
                Metadata Portal
              </span>
              <span>{portals[current].name}</span>
            </div>
            <ChevronIcon />
          </div>
        </Listbox.Button>
        <Transition
          as={Fragment}
          leave="transition ease-in duration-100"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <Listbox.Options className="absolute mt-1 left-0 right-0 overflow-auto rounded-lg bg-white p-2 text-base shadow-lg focus:outline-none z-10">
            {Object.keys(portals).map((portal) => (
              <Listbox.Option key={portal} value={portal}>
                {({ selected }) =>
                  selected ? (
                    <div
                      className={cn(
                        "flex items-center space-x-2 p-2 rounded-md hover:bg-neutral-100 transition-colors",
                        selected && "bg-neutral-100",
                        selected ? "cursor-default" : "cursor-pointer"
                      )}
                    >
                      {portals[portal].name}
                    </div>
                  ) : (
                    <a
                      className={cn(
                        "flex items-center space-x-2 p-2 rounded-md hover:bg-neutral-100 transition-colors",
                        selected && "bg-neutral-100",
                        selected ? "cursor-default" : "cursor-pointer"
                      )}
                      href={portals[portal].url}
                    >
                      {portals[portal].name}
                    </a>
                  )
                }
              </Listbox.Option>
            ))}
          </Listbox.Options>
        </Transition>
      </Listbox>
    </div>
  );
};
