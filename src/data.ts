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
    version: number,

}

export interface Chains {
    [name:string]: ChainSpec
}

export function getChains(): Chains {
    const chainList =  jsonData.map(chain => Object.assign({} as ChainSpec, chain))
    return chainList.reduce((obj, chain) => {
        return {
            ...obj,
            [chain.name]: chain
        }
    }, {});
}
