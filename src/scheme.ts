export interface ChainSpec {
  name: string;
  rpcEndpoint: string;
  genesisHash: string;
  unit: string;
  base58prefix: number;
  decimals: number;
  logo: string;
  metadataVersion: number;
  metadataQr: QrInfo;
  nextMetadataVersion: number | null;
  nextMetadataQr: QrInfo | null;
  specsQr: QrInfo;
}

export interface QrInfo {
  path: string;
  signedBy: string | null;
  color?: string;
}

export interface Chains {
  [name: string]: ChainSpec;
}
