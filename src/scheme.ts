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

export interface AddToSignerInterface {
  path: string;
  color: string;
  name: string;
}

export interface Chains {
  [name: string]: ChainSpec;
}

export type Portal = {
  name: string;
  url: string;
};
export interface Portals {
  [name: string]: Portal;
}
