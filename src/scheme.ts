export interface ChainSpec {
  name: string;
  color: string;
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
}

export interface AddToSignerInterface {
  path: string;
  color: string;
  name: string;
}

export interface Chains {
  [name: string]: ChainSpec;
}
