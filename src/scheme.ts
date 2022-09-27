export interface ChainSpec {
  title: string;
  color: string;
  rpcEndpoint: string;
  genesisHash: string;
  unit: string;
  base58prefix: number;
  decimals: number;
  icon: string;
  metadataVersion: number;
  metadataQr: QrInfo;
  nextMetadataVersion: number | null;
  nextMetadataQr: QrInfo | null;
  latestMetadata: string;
  specsQr: QrInfo;
  testnet: boolean;
}

export type SourceType = WasmSource | RpcSource | null;

export interface QrInfo {
  path: string;
  signedBy: string | null;
  source: SourceType;
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
