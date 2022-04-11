import { useState } from "react";
import { getChains } from "../scheme";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";
import {
  NetworkSlider,
  Network,
  getSubNetworkDetails,
  NetworkDetails,
  Card,
} from "mottled-library";
import "mottled-library/css/NetworkSlider.css";
import "mottled-library/css/Card.css";
import GitHub from "../assets/gh.png";

export default function App() {
  const allChains = getChains();
  const currentName = Object.keys(allChains)[0] || "polkadot";
  const chain = allChains[currentName];
  const metadataQr = chain.metadataQr;
  const specsQr = chain.specsQr;
  const svgClass = "inline mr-2 h-7";

  const [currentNetwork, setCurrentNetwork] = useState<
    NetworkDetails | undefined
  >(getSubNetworkDetails(currentName));

  document.body.style.backgroundColor = currentNetwork?.secondaryColor || "";

  return (
    <div className="flex flex-col">
      <div
        className="flex pt-8 pb-8 justify-around items-center"
        style={{ backgroundColor: currentNetwork?.primaryColor }}
      >
        <div className="text-white w-1 font-bold text-2xl">
          Metadata Update Portal
        </div>
        <NetworkSlider
          defaultNetwork={currentNetwork?.type}
          setNetwork={(network: NetworkDetails) => setCurrentNetwork(network)}
          networks={Object.keys(allChains) as Network[]}
        />
        <div className="text-white font-bold">
          <a
            className="w-5"
            href="https://github.com/paritytech/metadata-portal"
            target={"blank"}
          >
            <div className="flex float-left">Github</div>
            <div className="flex float-left">
              <img className="w-6 ml-2" src={GitHub} />
            </div>
          </a>
        </div>
      </div>
      <div className="flex flex-row flex-wrap justify-center pt-8">
        {metadataQr && (
          <Card st={{ minWidth: "500px;" }}>
            <div className="flex justify-between p-8">
              <h1
                className="text-xl sm:text-4xl"
                style={{ color: currentNetwork?.primaryColor }}
              >
                Metadata #{metadataQr.version}
              </h1>
              <div className="flex border-2 border-black rounded-xl p-2">
                {metadataQr.signedBy ? (
                  <div className="text-black font-normal">
                    <BadgeCheckIcon className={svgClass} />
                    Signed by {metadataQr.signedBy}
                  </div>
                ) : (
                  <div className="text-red-500">
                    <ExclamationCircleIcon className={svgClass} />
                    Unsigned
                  </div>
                )}
              </div>
            </div>
            <div className="flex">
              <QrCode path={metadataQr.path} />
              <div className="text-black overflow-auto p-5">
                <Specs
                  chainSpecs={{ ...chain }}
                  color={currentNetwork?.primaryColor}
                />
                {specsQr && <AddToSigner {...specsQr} />}
              </div>
            </div>
          </Card>
        )}
      </div>
    </div>
  );
}
