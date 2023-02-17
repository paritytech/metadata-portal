import { Listbox, Tab, Transition } from "@headlessui/react";
import { Fragment, useState } from "react";
import { ChainSpec, QrInfo, RpcSource, WasmSource } from "../scheme";
import { cn } from "../utils";
import Hash from "./Hash";
import { Row } from "./Row";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/24/solid";

type LabeledQr = {
  qr: QrInfo;
  label: string;
};

export const Network = ({ spec }: { spec: ChainSpec }) => {
  const [selectedIndex, setSelectedIndex] = useState(0);
  const qrs = [
    { qr: spec.metadataQr, label: `Current: #${spec.metadataVersion}` },
    spec.nextMetadataQr &&
      spec.nextMetadataVersion && {
        qr: spec.nextMetadataQr,
        label: `Next #${spec.nextMetadataVersion}`,
      },
  ].filter(Boolean) as LabeledQr[];
  if (spec.nextMetadataVersion && spec.nextMetadataQr) {
    qrs.push({
      qr: spec.nextMetadataQr,
      label: `Next #${spec.nextMetadataVersion}`,
    });
  }
  const [selectedIdx, setSelectedIdx] = useState(0);

  return (
    <div className="relative p-1">
      <div className="flex flex-col items-center p-4 pb-12">
        {selectedIndex === 0 && (
          <img
            className="w-full"
            src={process.env.PUBLIC_URL + spec.specsQr.path}
            alt="Qr code"
          />
        )}
        {selectedIndex === 1 && (
          <img
            className="w-full"
            src={process.env.PUBLIC_URL + qrs[selectedIdx].qr.path}
            alt="metadata qr code"
          />
        )}
        <div className="px-8 text-center text-sm text-neutral-500">
          {selectedIndex === 0 && "Scan this code to add chain specs to the "}
          {selectedIndex === 1 && "Scan this code to update "}
          <a
            href="https://parity.io/signer/"
            className="font-bold"
            target="_blank"
            rel="noreferrer"
          >
            Parity Signer App
          </a>
        </div>
      </div>
      <div className="relative p-2 pb-8">
        <Tab.Group selectedIndex={selectedIndex} onChange={setSelectedIndex}>
          <Tab.List className="flex bg-neutral-200 rounded-full p-1 mb-6">
            {["Chain Specs", "Metadata"].map((title) => (
              <Tab as={Fragment} key={title}>
                {({ selected }) => (
                  <button
                    className={cn(
                      "flex-1 p-3 rounded-full font-bold focus-visible:outline-none",
                      selected && "text-white"
                    )}
                    style={{ backgroundColor: selected ? spec.color : "" }}
                  >
                    {title}
                  </button>
                )}
              </Tab>
            ))}
          </Tab.List>
          <Tab.Panels>
            <Tab.Panel>
              <ul className="space-y-4">
                <Row title="RPC endpoint">{spec.rpcEndpoint}</Row>
                <Row title="Genesis hash">
                  <Hash value={spec.genesisHash} />
                </Row>
                <Row title="Address prefix">{spec.base58prefix}</Row>
                <Row title="Color">
                  <div className="flex space-x-2">
                    <div className="ml-2">{spec.color}</div>
                    <div
                      style={{ backgroundColor: spec.color }}
                      className="w-6 rounded-md"
                    />
                  </div>
                </Row>
                <Row title="Unit">{spec.unit}</Row>
              </ul>
            </Tab.Panel>
            <Tab.Panel>
              <div className="space-y-4">
                <Listbox value={selectedIdx} onChange={setSelectedIdx}>
                  <div className="relative w-full">
                    <Listbox.Button className="border border-gray-200 relative w-full cursor-default rounded-lg bg-white py-2 pl-3 pr-10 text-left shadow-md focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300">
                      <span className="block truncate">
                        <span className="font-bold">
                          {qrs[selectedIdx].label}
                        </span>
                        {qrs[selectedIdx].qr.signedBy ? (
                          <span className="p-1 mx-2 rounded-full text-xs text-green-700 bg-green-100">
                            Signed by {qrs[selectedIdx].qr.signedBy}
                          </span>
                        ) : (
                          <span className="p-1 mx-2 rounded-full text-red-700 bg-red-100">
                            Unsigned
                          </span>
                        )}
                      </span>
                      <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                        <ChevronUpDownIcon
                          className="h-5 w-5 text-gray-400"
                          aria-hidden="true"
                        />
                      </span>
                    </Listbox.Button>
                    <Transition
                      as={Fragment}
                      leave="transition ease-in duration-100"
                      leaveFrom="opacity-100"
                      leaveTo="opacity-0"
                    >
                      <Listbox.Options className="absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
                        {qrs.map((qr, idx) => (
                          <Listbox.Option
                            key={idx}
                            className={({ active }) =>
                              `relative cursor-default select-none py-2 pl-10 pr-4 ${
                                active
                                  ? "bg-amber-100 text-amber-900"
                                  : "text-gray-900"
                              }`
                            }
                            value={idx}
                          >
                            {({ selected }) => (
                              <>
                                <span
                                  className={`block truncate ${
                                    selected ? "font-medium" : "font-normal"
                                  }`}
                                >
                                  {qr.label}
                                  {qr.qr.signedBy ? (
                                    <span className="p-1 mx-2 rounded-full text-xs text-green-700 bg-green-100">
                                      Signed by {qr.qr.signedBy}
                                    </span>
                                  ) : (
                                    <span className="p-1 mx-2 rounded-full text-red-700 bg-red-100">
                                      Unsigned
                                    </span>
                                  )}
                                </span>
                                {selected ? (
                                  <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-amber-600">
                                    <CheckIcon
                                      className="h-5 w-5"
                                      aria-hidden="true"
                                    />
                                  </span>
                                ) : null}
                              </>
                            )}
                          </Listbox.Option>
                        ))}
                      </Listbox.Options>
                    </Transition>
                  </div>
                </Listbox>
                {qrs[selectedIdx].qr.source?.type === "Wasm" && (
                  <ul>
                    <Row title="Metadata source">
                      <a
                        href={`https://github.com/${
                          (qrs[selectedIdx].qr.source as WasmSource).github_repo
                        }/releases`}
                        target="_blank"
                        rel="noreferrer"
                      >
                        {(qrs[selectedIdx].qr.source as WasmSource).github_repo}
                      </a>
                    </Row>
                    <Row title="Blake2-256 hash">
                      <Hash
                        value={(qrs[selectedIdx].qr.source as WasmSource).hash}
                      />
                    </Row>
                  </ul>
                )}
                {qrs[selectedIdx].qr.source?.type === "Rpc" && (
                  <ul>
                    <Row title="Source block">
                      <Hash
                        value={(qrs[selectedIdx].qr.source as RpcSource).block}
                      />
                    </Row>
                  </ul>
                )}
              </div>
            </Tab.Panel>
          </Tab.Panels>
        </Tab.Group>
        <div
          className="absolute inset-0 rounded-[32px] pointer-events-none"
          style={{ backgroundColor: `${spec.color}0D` }}
        />
      </div>
      <div
        className="absolute inset-0 rounded-[32px] border-2 pointer-events-none"
        style={{
          backgroundColor: `${spec.color}0D`,
          borderColor: `${spec.color}1A`,
        }}
      />
    </div>
  );
};
