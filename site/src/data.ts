const chains: Chains = {
    polkadot: {
        name: "polkadot",
        iconPath: "icons/polkadot.ico",
        rpcEndpoint: "wss://polka.rpc.io",
        genesisHash: "0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
        color: "#e6007a",
        unit: "DOT",
        addressPrefix: 42,
        metadataQrCodes: [
            {
                path: "/qr/signed/load_metadata_polkadotV9140.apng",
                isVerified: true,
                title: "Metadata #9140"
            }
        ]
    },
    kusama: {
        name: "kusama",
        iconPath: "icons/kusama.ico",
        rpcEndpoint: "wss://kusama.rpc.io",
        genesisHash: "0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
        color: "#000000",
        unit: "KSM",
        addressPrefix: 2,
        metadataQrCodes: [
            {
                path: "/qr/signed/load_metadata_kusamaV9130.apng",
                isVerified: true,
                title: "Metadata #9130"
            }
        ]
    },
};

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
    return chains;
}
