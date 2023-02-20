import { Listbox, Transition } from "@headlessui/react";
import { Fragment } from "react";
import { PORTALS } from "../constants";
import { cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";

export const PortalSelect = () => (
  <div className="bordered-action">
    <Listbox as="div" value={PORTALS[0]} className="relative">
      <Listbox.Button className="block w-full text-left">
        <div className="text-sm text-neutral-500">Metadata Portal</div>
        <div className="flex items-center justify-between w-full text-lg">
          <span>{PORTALS[0]}</span>
          <ChevronIcon />
        </div>
      </Listbox.Button>
      <Transition
        as={Fragment}
        leave="transition ease-in duration-100"
        leaveFrom="opacity-100"
        leaveTo="opacity-0"
      >
        <Listbox.Options className="absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg focus:outline-none">
          {PORTALS.map((portal) => (
            <Listbox.Option key={portal} value={portal}>
              {({ selected }) => (
                <div
                  className={cn(
                    "flex items-center space-x-2 px-2 py-1",
                    selected && "bg-neutral-100"
                  )}
                >
                  <div className="text-xl">{portal}</div>
                </div>
              )}
            </Listbox.Option>
          ))}
        </Listbox.Options>
      </Transition>
    </Listbox>
  </div>
);
