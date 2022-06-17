/* eslint-disable no-empty-pattern */
import React from "react";
import QrCode from "./QrCode";
import Specs from "./Specs";
import AddToSigner from "./AddToSigner";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";
import NextMetadata from "./NextMetadata";
import { ChainSpec, QrInfo } from "../scheme";
import "./App.css";

interface Props {
  color: string;
  metadataQr: QrInfo;
  specs: ChainSpec;
  specsQr: QrInfo;
  chain: ChainSpec;
}

export default function Main({
  chain,
  color,
  metadataQr,
  specs,
  specsQr,
}: Props): JSX.Element {
  const svgClass = "inline mr-2 h-7";
  return (
    <div className="m-auto flex flex-col md:absolute md:left-60 md:pl-20 md:m-0">
      <div className="md:flex flex-row flex-wrap justify-center md:pt-8">
        <div
          className="px-2 py-2 rounded-lg border-gray-600 bg-white text-black"
          style={{ minWidth: "25rem" }}
        >
          <div className="flex justify-between mx-8 py-8 border-b-2 border-gray-200 items-center">
            <h1 className="text-lg md:text-2xl" style={{ color }}>
              Metadata #{chain.metadataVersion}
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
          <div className="px-12 justify-center pt-8">
            <QrCode {...metadataQr} />
            <div className="text-black py-5 w-full">
              <Specs chainSpecs={{ ...chain }} color={color} />
              <AddToSigner {...specsQr} color={color} />
              <NextMetadata {...specs} />
            </div>
          </div>
        </div>
      </div>
      <div className="flex w-full p-8 justify-evenly items-center">
        <a
          href="https://parity.io/signer/"
          target="_blank"
          className="text-black underline basis-40 m-1 text-center"
          rel="noreferrer"
        >
          Parity Signer
        </a>
        <a
          href="https://www.parity.io/"
          target="_blank"
          className="text-black underline basis-40 m-1 text-center"
          rel="noreferrer"
        >
          Developed by Parity
        </a>
      </div>
    </div>
  );
}
