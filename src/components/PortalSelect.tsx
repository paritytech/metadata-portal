import { Listbox, Transition } from "@headlessui/react";
import { Fragment } from "react";
import { PORTALS } from "../constants";
import { cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";

export const PortalSelect = () => (
  <div className="bordered-action">
    <Listbox as="div" value={PORTALS[0]} className="relative">
      <Listbox.Button className="flex items-center justify-between w-full text-xl">
        <div className="space-x-2">
          <span className="nowrap text-neutral-400">Metadata Portal</span>
          <span>{PORTALS[0]}</span>
        </div>
        <ChevronIcon />
      </Listbox.Button>
      <Transition
        as={Fragment}
        leave="transition ease-in duration-100"
        leaveFrom="opacity-100"
        leaveTo="opacity-0"
      >
        <Listbox.Options className="absolute mt-1 -left-4 -right-4 overflow-auto rounded-md bg-white py-2 text-base shadow-lg focus:outline-none">
          {PORTALS.map((portal) => (
            <Listbox.Option key={portal} value={portal}>
              {({ selected }) => (
                <div
                  className={cn(
                    "flex items-center space-x-2 px-4 py-2",
                    selected && "bg-neutral-100",
                    selected ? "cursor-default" : "cursor-pointer"
                  )}
                >
                  {portal}
                </div>
              )}
            </Listbox.Option>
          ))}
        </Listbox.Options>
      </Transition>
    </Listbox>
  </div>
);
