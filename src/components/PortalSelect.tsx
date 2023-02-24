import { Listbox, Transition } from "@headlessui/react";
import { Fragment } from "react";
import { PORTALS } from "../constants";
import { cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";

export const PortalSelect = () => (
  <div>
    <Listbox as="div" value={PORTALS[0]} className="relative">
      <Listbox.Button
        className={({ open }) =>
          cn(
            "w-full bordered-action py-3 hover:bg-neutral-100 transition-colors",
            open && "bg-neutral-100"
          )
        }
      >
        <div className="flex items-center justify-between -my-px">
          <div className="space-x-2 text-2xl">
            <span className="nowrap text-black opacity-70">
              Metadata Portal
            </span>
            <span>{PORTALS[0]}</span>
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
        <Listbox.Options className="absolute mt-1 left-0 right-0 overflow-auto rounded-lg bg-white p-2 text-base shadow-lg focus:outline-none">
          {PORTALS.map((portal) => (
            <Listbox.Option key={portal} value={portal}>
              {({ selected }) => (
                <div
                  className={cn(
                    "flex items-center space-x-2 p-2 rounded-md text-2xl hover:bg-neutral-100 transition-colors",
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
