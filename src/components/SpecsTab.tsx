import { ReactElement } from "react";
import { ChainSpec } from "../scheme";
import "./SpecsTab.css";
import Hash from "./Hash";
import { ExternalLinkIcon } from "@heroicons/react/outline";
import { getBackgroundStyle } from "../utils";

interface SpecsProps {
  specs: ChainSpec;
}

export default function SpecsTab({ specs }: SpecsProps) {
  const {
    color,
    rpcEndpoint,
    genesisHash,
    unit,
    base58prefix,
    specsQr,
    latestMetadata,
  } = specs;

  return (
    <div className="px-2">
      <div className="py-5 border-b border-neutral-300">
        <ul>{row("RPC endpoint", <>{rpcEndpoint}</>, true)}</ul>
        <ul className="flex">
          {row("Genesis hash", <Hash value={genesisHash} />)}
          {row("Address prefix", base58prefix.toString())}
        </ul>
        <ul className="flex">
          {row(
            "Color",
            <>
              <div
                style={getBackgroundStyle(color)}
                className="w-6 rounded-md border-none block"
              ></div>
              <div className="ml-2">{color}</div>
            </>
          )}
          {row("Unit", unit)}
        </ul>
      </div>

      <div className="py-5 border-b border-neutral-300">
        <ul>
          {row(
            "Latest metadata QR",
            <>
              <a
                href={process.env.PUBLIC_URL + "/" + latestMetadata}
                target="_blank"
                className="font-medium"
                style={{ color: `${color}` }}
                rel="noreferrer"
              >
                {`${location.origin}/${
                  process.env.PUBLIC_URL + "/" + latestMetadata
                }`}
                <ExternalLinkIcon className={"inline w-4 h-4 ml-1"} />
              </a>
            </>,
            true
          )}
        </ul>
      </div>

      <div className="pt-3">
        {"Scan this code to add chain specs to the "}
        <a
          href="https://parity.io/signer/"
          target="_blank"
          className="font-medium"
          style={{ color: `${color}` }}
          rel="noreferrer"
        >
          Parity Signer App
          <ExternalLinkIcon className={"inline w-4 h-4 ml-1"} />
        </a>
      </div>

      <div className="flex justify-center">
        <img src={process.env.PUBLIC_URL + "/" + specsQr.path} alt="Qr code" />
      </div>
    </div>
  );
}

export function row(
  title: string,
  content: ReactElement | string,
  fullWidth = false,
  color = "#000000"
) {
  const liSize = fullWidth ? "py-1" : "py-1 w-1/2";
  return (
    <li className={liSize}>
      <div className="text-sm text-black">{title}</div>
      <div className="flex" style={{ color }}>
        {content}
      </div>
    </li>
  );
}
