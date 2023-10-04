import { Tab } from "@headlessui/react";
import { Fragment, useState } from "react";
import { ChainSpec, RpcSource, WasmSource } from "../scheme";
import { cn, formatTitle } from "../utils";
import Copyable, { hashSlicer, keepHeadSlicer } from "./Copyable";
import { Hr } from "./Hr";
import { Links } from "./Links";
import { Row } from "./Row";
import { icon } from "../icons";

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

export const Network = ({
  spec,
  chainPortalId,
}: {
  spec: ChainSpec;
  chainPortalId: string;
}) => {
  const [selectedTab, setSelectedTab] = useState(tabFromSearch());
  const metadataQr = spec.metadataQr;

  function updateTab(v: number) {
    setTabToSearch(v);
    setSelectedTab(v);
  }

  const vaultLink = (
    <a
      href="https://parity.io/signer/"
      className="font-bold"
      target="_blank"
      rel="noreferrer"
    >
      Polkadot Vault
    </a>
  );

  const createGithubIssueLink = (
    <a
      href="https://github.com/opentensor/metadata-portal/issues/new"
      className="block mt-10 font-extrabold"
      style={{
        color: `${spec.color}`,
      }}
      target="_blank"
      rel="noreferrer"
    >
      Create a Github issue
    </a>
  );

  return (
    <div>
      <div className="hidden xl:flex items-center justify-between mb-10">
        <div className="flex items-center space-x-2 text-[40px] leading-none unbounded">
          <img
            src={icon(chainPortalId)}
            className="w-14 h-14 rounded-full bg-neutral-200"
          />
          <span>
            {formatTitle(
              spec.title === "node-subtensor" ? "Bittensor" : spec.title,
            )}
          </span>
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
          <div className="w-full max-w-xs">
            {selectedTab === 0 && (
              <div>
                <img
                  className="w-full"
                  src={process.env.PUBLIC_URL + spec.specsQr.path}
                  alt="Qr code"
                />
                <div className="text-center text-sm text-black opacity-50">
                  {"Scan this code to add chain specs to the "}
                  {vaultLink}
                </div>
              </div>
            )}
            {selectedTab === 1 && (
              <div>
                {!metadataQr && (
                  <div className="flex aspect-square text-center">
                    <div className="m-auto">
                      The metadata for {spec.title} Network is out of date.
                      Request the new metadata version by creating a Github
                      issue.
                      {createGithubIssueLink}
                    </div>
                  </div>
                )}
                {metadataQr && (
                  <div>
                    <img
                      className="w-full"
                      src={process.env.PUBLIC_URL + metadataQr?.file.path}
                      alt="metadata qr code"
                    />
                    <div className="text-center text-sm text-black opacity-50">
                      {"Scan this code to update "}
                      {vaultLink}
                    </div>
                  </div>
                )}
              </div>
            )}
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
                        selected && "text-white",
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
                  <Row title="RPC endpoint">
                    <Copyable
                      value={spec.rpcEndpoint}
                      slicer={keepHeadSlicer(25)}
                    />
                  </Row>
                  <Row title="Genesis hash">
                    <Copyable value={spec.genesisHash} slicer={hashSlicer} />
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
                {metadataQr && (
                  <div className="space-y-6">
                    {metadataQr?.file.source?.type === "Wasm" && (
                      <ul className="space-y-6">
                        <Row title="Metadata source">
                          <a
                            href={`https://github.com/${
                              (metadataQr?.file.source as WasmSource)
                                .github_repo
                            }/releases`}
                            target="_blank"
                            rel="noreferrer"
                          >
                            {
                              (metadataQr?.file.source as WasmSource)
                                .github_repo
                            }
                          </a>
                        </Row>
                        <Row title="Hash">
                          <Copyable
                            value={(metadataQr?.file.source as WasmSource).hash}
                            slicer={hashSlicer}
                          />
                        </Row>
                      </ul>
                    )}
                    {metadataQr?.file.source?.type === "Rpc" && (
                      <ul className="space-y-4">
                        <Row title="Source block">
                          <Copyable
                            value={(metadataQr?.file.source as RpcSource).block}
                            slicer={hashSlicer}
                          />
                        </Row>
                      </ul>
                    )}
                    <Row title="Metadata Version">#{metadataQr?.version}</Row>
                    <Row title="Signed by">{metadataQr?.file.signedBy}</Row>
                  </div>
                )}
              </Tab.Panel>
            </Tab.Panels>
          </Tab.Group>
        </div>
      </div>
    </div>
  );
};
