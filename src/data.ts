import jsonData from './chains.json';

export interface ChainSpec {
    name: string,
    iconPath: string,
    rpcEndpoint: string,
    genesisHash: string,
    color: string,
    unit: string,
    addressPrefix: number,
    metadataQrCodes: QrInfo[]
}

export interface QrInfo {
    path: string,
    isVerified: boolean,
    title: string,

}

export interface Chains {
    [name:string]: ChainSpec
}

export function getChains(): Chains {
    return Object.assign({} as Chains, jsonData)
}
