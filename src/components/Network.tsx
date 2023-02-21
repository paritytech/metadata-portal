import { Listbox, Tab, Transition } from "@headlessui/react";
import { Fragment, useState } from "react";
import { ChainSpec, QrInfo, RpcSource, WasmSource } from "../scheme";
import { capitalizeFirstLetter, cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";
import Hash from "./Hash";
import { Links } from "./Links";
import { Row } from "./Row";

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
    <>
      <div className="hidden lg:flex items-start justify-between">
        <div className="text-6xl">{capitalizeFirstLetter(spec.title)}</div>
        <Links />
      </div>
      <div
        className="flex flex-col lg:flex-row p-2 border-2 rounded-4xl space-y-2 lg:space-y-0 lg:space-x-2"
        style={{
          backgroundColor: `${spec.color}0D`,
          borderColor: `${spec.color}1A`,
        }}
      >
        <div className="flex flex-col items-center p-2 lg:p-8 pb-12 lg:w-1/2 bg-white rounded-3xl">
          <div className="w-full max-w-lg aspect-square bg-neutral-100">
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
          </div>
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
        <div className="p-2 lg:p-8 pb-12 lg:w-1/2 bg-white rounded-3xl">
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
                  <Listbox
                    as="div"
                    className="relative w-full"
                    value={selectedIdx}
                    onChange={setSelectedIdx}
                  >
                    <Listbox.Button className="bordered-action flex items-center justify-between w-full space-x-4">
                      <span className="flex items-center space-x-2">
                        <span>{qrs[selectedIdx].label}</span>
                        <span className="px-2 py-1 text-sm rounded-full bg-neutral-200">
                          {qrs[selectedIdx].qr.signedBy
                            ? `Signed by ${qrs[selectedIdx].qr.signedBy}`
                            : "Unsigned"}
                        </span>
                      </span>
                      <ChevronIcon />
                    </Listbox.Button>
                    <Transition
                      as={Fragment}
                      leave="transition ease-in duration-100"
                      leaveFrom="opacity-100"
                      leaveTo="opacity-0"
                    >
                      <Listbox.Options className="absolute mt-1 left-0 right-0 overflow-auto rounded-md bg-white py-2 text-base shadow-lg focus:outline-none">
                        {qrs.map((qr, idx) => (
                          <Listbox.Option key={idx} value={idx}>
                            {({ selected }) => (
                              <div
                                className={cn(
                                  "flex items-center space-x-2 px-4 py-2",
                                  selected && "bg-neutral-100",
                                  selected ? "cursor-default" : "cursor-pointer"
                                )}
                              >
                                <span>{qr.label}</span>
                                <span className="px-2 py-1 text-sm rounded-full bg-neutral-200">
                                  {qrs[selectedIdx].qr.signedBy
                                    ? `Signed by ${qrs[selectedIdx].qr.signedBy}`
                                    : "Unsigned"}
                                </span>
                              </div>
                            )}
                          </Listbox.Option>
                        ))}
                      </Listbox.Options>
                    </Transition>
                  </Listbox>
                  {qrs[selectedIdx].qr.source?.type === "Wasm" && (
                    <ul className="space-y-4">
                      <Row title="Metadata source">
                        <a
                          href={`https://github.com/${
                            (qrs[selectedIdx].qr.source as WasmSource)
                              .github_repo
                          }/releases`}
                          target="_blank"
                          rel="noreferrer"
                        >
                          {
                            (qrs[selectedIdx].qr.source as WasmSource)
                              .github_repo
                          }
                        </a>
                      </Row>
                      <Row title="Blake2-256 hash">
                        <Hash
                          value={
                            (qrs[selectedIdx].qr.source as WasmSource).hash
                          }
                        />
                      </Row>
                    </ul>
                  )}
                  {qrs[selectedIdx].qr.source?.type === "Rpc" && (
                    <ul className="space-y-4">
                      <Row title="Source block">
                        <Hash
                          value={
                            (qrs[selectedIdx].qr.source as RpcSource).block
                          }
                        />
                      </Row>
                    </ul>
                  )}
                </div>
              </Tab.Panel>
            </Tab.Panels>
          </Tab.Group>
        </div>
      </div>
    </>
  );
};
