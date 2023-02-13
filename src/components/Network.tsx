import { ChainSpec } from "../scheme";
import { Tab } from "@headlessui/react";
import MetadataTab from "./MetadataTab";
import SpecsTab from "./SpecsTab";
import { cn } from "../utils";

export const Network = ({ spec }: { spec: ChainSpec }) => {
  return (
    <Tab.Group
      as="div"
      className="bg-neutral-100 rounded-3xl p-4 max-w-md space-y-4"
    >
      <Tab.List className="flex bg-neutral-200 rounded-full p-2">
        {["Chain", "Metadata"].map((title) => (
          <Tab
            key={title}
            className={({ selected }) =>
              cn(
                "flex-1 p-3 font-bold rounded-full focus-visible:outline-none",
                selected && "bg-black text-white"
              )
            }
          >
            {title}
          </Tab>
        ))}
      </Tab.List>
      <Tab.Panels>
        <Tab.Panel>
          <SpecsTab specs={{ ...spec }} />
        </Tab.Panel>
        <Tab.Panel className="flex justify-center">
          <MetadataTab specs={{ ...spec }} />
        </Tab.Panel>
      </Tab.Panels>
    </Tab.Group>
  );
};
