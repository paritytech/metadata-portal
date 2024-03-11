import { useEffect, useState } from "react";
import { ArrowUpTrayIcon } from "@heroicons/react/24/solid";
import { web3Enable } from "@polkadot/extension-dapp";
import {
  InjectedExtension,
  InjectedMetadataKnown,
  MetadataDef,
} from "@polkadot/extension-inject/types";
import { ChainSpec } from "../scheme";
import Dropdown from "./Dropdown";
import Button from "./Button";
import { capitalizeFirstLetter } from "../utils";

export default function Extension(chainSpec: ChainSpec) {
  const [selected, setSelected] = useState<InjectedExtension | undefined>(
    undefined
  );
  const [extensions, setExtensions] = useState<InjectedExtension[]>([]);
  useEffect(() => {
    extensionsToUpdate(chainSpec).then((injected) => {
      setExtensions(injected);
      setSelected(injected[0]);
    });
  }, [chainSpec]);

  if (!selected) {
    return null;
  }

  const meta: MetadataDef = {
    chain: capitalizeFirstLetter(chainSpec.title),
    genesisHash: chainSpec.genesisHash,
    icon: chainSpec.logo,
    specVersion: chainSpec.liveMetaVersion,
    ss58Format: chainSpec.base58prefix,
    tokenDecimals: chainSpec.decimals,
    tokenSymbol: chainSpec.unit,
    chainType: "substrate",
    types: {} as unknown as Record<string, string>,
  };

  return (
    <div className="mt-5 w-48">
      <Dropdown<InjectedExtension>
        selected={selected}
        all={extensions}
        onChange={setSelected}
        formatter={(item) => `${item.name} ${item.version}`}
        label="Upgradable extensions:"
        actionButton={
          <Button
            label={<ArrowUpTrayIcon className="w-5 h-5" />}
            onClick={() => {
              selected?.metadata?.provide(meta).then((ok) => {
                if (ok) {
                  extensionsToUpdate(chainSpec).then((injected) => {
                    setExtensions(injected);
                    setSelected(injected[0]);
                  });
                }
              });
            }}
          />
        }
      />
    </div>
  );
}

interface ExtensionWithMeta {
  injectedExtension: InjectedExtension;
  metadata: InjectedMetadataKnown[];
}

async function extensionsToUpdate(
  chainSpec: ChainSpec
): Promise<InjectedExtension[]> {
  const allInjected = await web3Enable("Metadata Portal");

  const extensions = await Promise.all(
    allInjected.map(async (injected) => {
      const metas = await injected.metadata?.get();
      return {
        injectedExtension: injected,
        metadata: metas,
      } as ExtensionWithMeta;
    })
  );

  return extensions
    .filter((extension) => {
      const current = extension.metadata.find(
        ({ genesisHash }) => genesisHash === chainSpec.genesisHash
      );
      if (!current) {
        return true;
      }
      return current.specVersion < chainSpec.liveMetaVersion;
    })
    .map((extension) => extension.injectedExtension);
}
