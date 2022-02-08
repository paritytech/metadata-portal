import {Chains} from "../scheme";
import Selector from "./Selector";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";

interface Props{
    allChains: Chains
    currentName: string
}

export default function Page({allChains, currentName}: Props) {
    const chain = allChains[currentName]
    const metadataQr = chain.metadataQr;
    document.body.style.backgroundColor = chain.color;
    return (
        <div className="flex flex-col m-6">
            <div className="flex justify-center mb-8">
                <Selector selectedName={currentName} allChains={allChains}/>
            </div>
            <div className="flex flex-row flex-wrap justify-center gap-10">
                { metadataQr && (
                    <>
                        <QrCode {...metadataQr} />
                        <div className="text-white">
                            <h1 className="text-5xl mb-5">Metadata #{metadataQr.version}</h1>
                            <Specs {...chain} />
                            <AddToSigner />
                        </div>
                    </>
                )}
            </div>
        </div>
    )
}
