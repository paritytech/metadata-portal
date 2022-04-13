import { useEffect, useState } from "react";
import { ChainSpec, getChains, QrInfo } from "../scheme";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";
import {
  NetworkSlider,
  Network,
  getChain,
  NetworkDetails,
  Card,
} from "mottled-library";
import "mottled-library/css/NetworkSlider.css";
import "mottled-library/css/Card.css";
import GitHub from "../assets/gh.png";

export default function App() {
  const allChains = getChains();
  const currentName = Object.keys(allChains)[0] || "polkadot";
  const svgClass = "inline mr-2 h-7";
  const [currentNetwork, setCurrentNetwork] = useState<
    NetworkDetails | undefined
  >(getChain(currentName));
  const [metadataQr, setMetadataQr] = useState<QrInfo | undefined>(
    allChains[currentName].metadataQr
  );

  const [specsQr, setSpecsQr] = useState<QrInfo | undefined>(
    allChains[currentName].specsQr
  );

  const [chain, setChain] = useState<ChainSpec>(allChains[currentName]);

  useEffect(() => {
    const name = currentNetwork?.name?.toLocaleLowerCase();
    if (name) {
      setChain(allChains[name]);
      setMetadataQr(allChains[name]?.metadataQr);
      setSpecsQr(allChains[name]?.specsQr);
    }
  }, [currentNetwork?.name]);

  document.body.style.backgroundColor = currentNetwork?.secColor || "";

  return (
    <div className="flex flex-col">
      <div
        className="lg:flex lg:p-8 p-5 justify-around items-center"
        style={{ backgroundColor: currentNetwork?.color }}
      >
        <div className="text-white lg:w-1 font-bold text-2xl mb-5 lg:text-left text-center">
          Metadata Update Portal
        </div>
        <NetworkSlider
          defaultNetwork={currentNetwork?.type}
          setNetwork={(network: NetworkDetails) => setCurrentNetwork(network)}
          networks={Object.keys(allChains) as Network[]}
        />
        <div className="text-white font-bold">
          <a
            className="lg:text-left text-center lg:mt-0 mt-5 lg:block inline-block lg:w-fit w-full"
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
      <div className="md:flex flex-row flex-wrap justify-center lg:pt-8">
        {metadataQr && (
          <Card>
            <div className="flex justify-between mx-8 py-8 border-b-2 border-gray-200 ">
              <h1
                className="text-2xl lg:text-4xl"
                style={{ color: currentNetwork?.color }}
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
            <div className="lg:flex grid justify-center pt-8">
              <QrCode path={metadataQr.path} />
              <div className="text-black overflow-auto p-5 w-72">
                <Specs
                  chainSpecs={{ ...chain }}
                  color={currentNetwork?.color}
                />
                {specsQr && <AddToSigner {...specsQr} />}
              </div>
            </div>
          </Card>
        )}
      </div>
      <div className="flex w-full pt-8 pb-8 justify-evenly items-center">
        <a
          href="https://www.parity.io/"
          target="_blank"
          className="text-white underline"
          rel="noreferrer"
        >
          Developed by Parity
        </a>
        <a
          href="https://www.parity.io/terms/"
          target="_blank"
          className="text-white underline"
          rel="noreferrer"
        >
          Terms of Service
        </a>
      </div>
    </div>
  );
}
