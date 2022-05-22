import { useEffect, useState } from "react";
import {Chains} from "../scheme";
import { useLocation } from "react-router-dom";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";
import {
  useLocalStorage,
  getChain,
  Card,
  NetworkSlider,
  Network,
  NetworkDetails,
} from "mottled-library";
import "mottled-library/css/NetworkSlider.css";
import "mottled-library/css/Card.css";
import GitHub from "../assets/gh.png";
import NextMetadata from "./NextMetadata";
// import Extension from "./Extension";

export default function App() {
  const [localNetwork, setLocalNetwork] = useLocalStorage("chosenNetwork");
  const svgClass = "inline mr-2 h-7";

  const [allChains, setAllChains] = useState<Chains>({});
  useEffect(() => {
    const fetchData = async () => {
      const data = await fetch("data.json")
          .then(response => response.json())
          .catch(e => {
            console.error("Unable to fetch data file. Run `make collector` to generate it")
            return e;
          });
      return await data as Chains;
    };
    fetchData().then(r => setAllChains(r));
  }, [])

  // replace existing url hash in order to identify the network
  // from the url if it exists (it prioritizes over every other option below)
  const location = useLocation().hash.replace("#/", "");
  // check if URL exists in given Networks, if not
  // check localStorage if it contains a - from before - chosen network, if not
  // retrieve the 1st available network from the given ones, else (rare and wrong case)
  // default to polkadot
  const [chain, setChain] = useState<string>(
      location ||
      localNetwork && localNetwork.toLowerCase() ||
      Object.keys(allChains)[0] ||
      "polkadot"
  );

  const specs = allChains[chain];
  if (!specs){
    return null
  }
  const extraInfo = getChain(chain);
  document.body.style.backgroundColor = extraInfo?.secColor || "";

  const onNetworkSelect = (network: NetworkDetails) => {
    setChain(network.name.toLowerCase());
    setLocalNetwork(network.name);
    window.location.assign("#/" + network.name.toLowerCase())
  };

  return (
      <div className="flex flex-col">
        <div
            className="lg:flex justify-around p-2 items-center"
            style={{backgroundColor: extraInfo?.color}}
        >
          <div className="text-white lg:w-1 font-bold text-2xl lg:text-left text-center">
            Metadata Update Portal
          </div>
          <div className="lg:mt-0 mt-5 max-w-base lg:max-w-2xl">
            <NetworkSlider
                defaultNetwork={chain as Network}
                setNetwork={onNetworkSelect}
                networks={Object.keys(allChains) as Network[]}
            />
          </div>
          <div className="text-white font-bold">
            <a
                className="lg:text-left text-center lg:mt-0 mt-5 lg:block inline-block lg:w-fit w-full"
                href="https://github.com/paritytech/metadata-portal"
                target={"blank"}
            >
              <div className="flex float-left">Github</div>
              <div className="flex float-left">
                <img className="w-6 ml-2" src={GitHub}/>
              </div>
            </a>
          </div>
        </div>
        <div className="md:flex flex-row flex-wrap justify-center lg:pt-8">
          <Card>
            <div className="flex justify-between mx-8 py-8 border-b-2 border-gray-200 ">
              <h1
                  className="text-2xl lg:text-4xl"
                  style={{color: extraInfo?.color}}
              >
                Metadata #{specs.metadataVersion}
              </h1>
              <div className="flex border-2 border-black rounded-xl p-2">
                {specs.metadataQr.signedBy ? (
                    <div className="text-black font-normal">
                      <BadgeCheckIcon className={svgClass}/>
                      Signed by {specs.metadataQr.signedBy}
                    </div>
                ) : (
                    <div className="text-red-500">
                      <ExclamationCircleIcon className={svgClass}/>
                      Unsigned
                    </div>
                )}
              </div>
            </div>
            <div className="lg:flex grid justify-center pt-8">
              <QrCode {...specs.metadataQr} />
              <div className="text-black p-5 w-72">
                <Specs chainSpecs={{...specs}} color={extraInfo?.color}/>
                <AddToSigner {...specs.specsQr} />
                <NextMetadata {...specs} />
                {/*<Extension {...chain} />*/}
              </div>
            </div>
          </Card>
        </div>
        <div className="flex w-full p-8 justify-evenly items-center">
          <a
              href="https://parity.io/signer/"
              target="_blank"
              className="text-white underline basis-40 m-1 text-center"
              rel="noreferrer"
          >
            Parity Signer
          </a>
          <a
              href="https://www.parity.io/"
              target="_blank"
              className="text-white underline basis-40 m-1 text-center"
              rel="noreferrer"
          >
            Developed by Parity
          </a>
          <a
              href="https://www.parity.io/terms/"
              target="_blank"
              className="text-white underline basis-40 m-1 text-center"
              rel="noreferrer"
          >
            Terms of Service
          </a>
        </div>
      </div>
  );
}
