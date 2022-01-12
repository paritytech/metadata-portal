import {Chains} from "../data";
import Selector from "./Selector";
import QrCode from "./QrCode";
import Specs from "./Specs";

interface Props{
    allChains: Chains
    currentName: string
}

export default function InfoPage({allChains, currentName}: Props) {
    const chain = allChains[currentName]
    const latestQr = chain.metadataQrCodes[0]
    document.body.style.backgroundColor = chain.color;
    return (
        <div className="flex flex-col m-6">
            <div className="flex justify-center mb-8">
                <Selector selectedName={currentName} allChains={allChains}/>
            </div>
            <div className="flex flex-row flex-wrap justify-center gap-10">
                <QrCode {...latestQr}/>
                <div className="text-white">
                    <h1 className="text-4xl mb-5">Metadata #{latestQr.version}</h1>
                    <Specs {...chain}/>
                </div>
            </div>
        </div>
    )
}

