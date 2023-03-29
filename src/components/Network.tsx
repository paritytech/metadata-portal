import { Listbox, Tab, Transition } from "@headlessui/react";
import { Fragment, useState } from "react";
import { ChainSpec, QrInfo, RpcSource, WasmSource } from "../scheme";
import { capitalizeFirstLetter, cn } from "../utils";
import { ChevronIcon } from "./ChevronIcon";
import Hash from "./Hash";
import { Hr } from "./Hr";
import { Links } from "./Links";
import { Row } from "./Row";
import { icon } from "../icons";

type LabeledQr = {
  qr: QrInfo;
  label: string;
};

function tabFromSearch() {
  const params = new URLSearchParams(location.search);
  const tab = parseInt(params.get("tab") || "0", 10);

  return tab === 0 || tab === 1 ? tab : 0;
}

function setTabToSearch(v: number) {
  const url = new URL(location.href);
  url.searchParams.set("tab", v.toString());
  window.history.replaceState(null, "", url);
}

export const Network = ({ spec }: { spec: ChainSpec }) => {
  const [selectedTab, setSelectedTab] = useState(tabFromSearch());
  const [selectedQr, setSelectedQr] = useState(0);
  const qrs = spec.metadataQrs.map(qr => ({
    qr: qr.file,
    label: `${capitalizeFirstLetter(qr.status.toString())} #${qr.version}`
  } as LabeledQr));

  function updateTab(v: number) {
    setTabToSearch(v);
    setSelectedTab(v);
  }

  return (
    <div>
      <div className="hidden xl:flex items-center justify-between mb-10">
        <div className="flex items-center space-x-2 text-[40px] leading-none unbounded">
          <img
            src={icon(spec.title)}
            className="w-14 h-14 rounded-full bg-neutral-200"
          />
          <span>{capitalizeFirstLetter(spec.title)}</span>
        </div>
        <Links />
      </div>
      <div
        className="flex flex-col md:flex-row p-2 border-2 rounded-4xl space-y-2 md:space-y-0 md:space-x-2"
        style={{
          backgroundColor: `${spec.color}0D`,
          borderColor: `${spec.color}1A`,
        }}
      >
        <div className="flex flex-col items-center p-10 md:w-[55%] bg-white rounded-3xl">
          <div className="w-full max-w-xs aspect-square bg-neutral-100">
            {selectedTab === 0 && (
              <img
                className="w-full"
                src={process.env.PUBLIC_URL + spec.specsQr.path}
                alt="Qr code"
              />
            )}
            {selectedTab === 1 && (
              <img
                className="w-full"
                src={process.env.PUBLIC_URL + qrs[selectedQr].qr.path}
                alt="metadata qr code"
              />
            )}
          </div>
          <div className="text-center text-sm text-black opacity-50">
            <div>
              {selectedTab === 0 && "Scan this code to add chain specs to the "}
              {selectedTab === 1 && "Scan this code to update "}
            </div>
            <a
              href="https://parity.io/signer/"
              className="font-bold"
              target="_blank"
              rel="noreferrer"
            >
              Polkadot Vault
            </a>
          </div>
        </div>
        <div className="p-2 md:p-4 md:w-[45%] bg-white rounded-3xl">
          <Tab.Group selectedIndex={selectedTab} onChange={updateTab}>
            <Tab.List className="flex bg-neutral-200 rounded-full p-1">
              {["Chain Specs", "Metadata"].map((title) => (
                <Tab as={Fragment} key={title}>
                  {({ selected }) => (
                    <button
                      className={cn(
                        "flex-1 p-3 rounded-full focus:outline-none",
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
            <div className="m-4">
              <Hr />
            </div>
            <Tab.Panels className="m-4 mt-6 mb-4">
              <Tab.Panel>
                <ul className="space-y-6">
                  <Row title="RPC endpoint">{spec.rpcEndpoint}</Row>
                  <Row title="Genesis hash">
                    <Hash value={spec.genesisHash} />
                  </Row>
                  <Row title="Color">
                    <div className="flex space-x-2">
                      <div className="ml-2">{spec.color}</div>
                      <div
                        style={{ backgroundColor: spec.color }}
                        className="w-6 rounded-md"
                      />
                    </div>
                  </Row>
                  <Row title="Address prefix">{spec.base58prefix}</Row>
                  <Row title="Unit">{spec.unit}</Row>
                  <Row title="Latest metadata">{spec.liveMetaVersion}</Row>
                </ul>
              </Tab.Panel>
              <Tab.Panel>
                <div className="space-y-6">
                  <Listbox
                    as="div"
                    className="relative w-full"
                    value={selectedQr}
                    onChange={setSelectedQr}
                  >
                    <Listbox.Button className="bordered-action flex items-center justify-between w-full space-x-4">
                      <span className="flex items-center space-x-2">
                        <span>{qrs[selectedQr].label}</span>
                        <span className="px-2 py-1 text-sm rounded-full bg-neutral-200">
                          {qrs[selectedQr].qr.signedBy
                            ? `Signed by ${qrs[selectedQr].qr.signedBy}`
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
                      <Listbox.Options className="absolute mt-1 left-0 right-0 overflow-auto rounded-lg bg-white p-2 text-base shadow-lg focus:outline-none z-10">
                        {qrs.map((qr, idx) => (
                          <Listbox.Option key={idx} value={idx}>
                            {({ selected }) => (
                              <div
                                className={cn(
                                  "flex items-center space-x-2 p-2 rounded-md hover:bg-neutral-100 transition-colors",
                                  selected && "bg-neutral-100",
                                  selected ? "cursor-default" : "cursor-pointer"
                                )}
                              >
                                <span>{qr.label}</span>
                                <span className="px-2 py-1 text-sm rounded-full bg-neutral-200">
                                  {qrs[selectedQr].qr.signedBy
                                    ? `Signed by ${qrs[selectedQr].qr.signedBy}`
                                    : "Unsigned"}
                                </span>
                              </div>
                            )}
                          </Listbox.Option>
                        ))}
                      </Listbox.Options>
                    </Transition>
                  </Listbox>
                  {qrs[selectedQr].qr.source?.type === "Wasm" && (
                    <ul className="space-y-6">
                      <Row title="Metadata source">
                        <a
                          href={`https://github.com/${
                            (qrs[selectedQr].qr.source as WasmSource)
                              .github_repo
                          }/releases`}
                          target="_blank"
                          rel="noreferrer"
                        >
                          {
                            (qrs[selectedQr].qr.source as WasmSource)
                              .github_repo
                          }
                        </a>
                      </Row>
                      <Row title="Hash">
                        <Hash
                          value={(qrs[selectedQr].qr.source as WasmSource).hash}
                        />
                      </Row>
                    </ul>
                  )}
                  {qrs[selectedQr].qr.source?.type === "Rpc" && (
                    <ul className="space-y-4">
                      <Row title="Source block">
                        <Hash
                          value={(qrs[selectedQr].qr.source as RpcSource).block}
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
    </div>
  );
};
