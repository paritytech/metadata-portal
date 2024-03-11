import jsonData from "./chains.json"; // Dynamically generated datafile. Run `make collector` to create

export interface ChainSpec {
  title: string;
  color: string;
  rpcEndpoint: string;
  genesisHash: string;
  unit: string;
  base58prefix: number;
  decimals: number;
  logo: string;
  liveMetaVersion: number;
  metadataQr?: MetadataQr;
  latestMetadata: string;
  specsQr: QrInfo;
  relayChain?: string;
}

export type SourceType = WasmSource | RpcSource | null;

export interface QrInfo {
  path: string;
  signedBy: string | null;
  source: SourceType;
}

export interface MetadataQr {
  version: number;
  file: QrInfo;
  status: string;
}

interface SourceBase {
  type: string;
}

export interface WasmSource extends SourceBase {
  github_repo: string;
  hash: string;
}

export interface RpcSource extends SourceBase {
  block: string;
}

export interface Chains {
  [title: string]: ChainSpec;
}

export function getChains(): Chains {
  const chainList = Object.values(jsonData).map((chain: object) =>
    Object.assign({} as ChainSpec, chain)
  );
  return chainList.reduce((obj: Chains, chain: ChainSpec) => {
    return {
      ...obj,
      [chain.title]: chain,
    };
  }, {});
}
