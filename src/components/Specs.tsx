import { ReactElement } from "react";
import { ChainSpec } from "../scheme";
import "./Specs.css";
import Hash from "./Hash";

interface SpecsProps {
  chainSpecs: ChainSpec;
  color?: string;
}

export default function Specs({ chainSpecs, color }: SpecsProps) {
  const { rpcEndpoint, genesisHash, unit, base58prefix } = chainSpecs;

  return (
    <>
      <ul>
        {row(
          "RPC endpoint",
          <div className="font-bold" style={{ color: color }}>
            {rpcEndpoint}
          </div>,
          true
        )}
      </ul>
      <ul className="flex">
        {row("Genesis hash", <Hash value={genesisHash} />  )}
        {row("Address prefix", base58prefix.toString())}
      </ul>
      <ul className="flex">
        {row(
          "Color",
          <>
            <div
              style={{ backgroundColor: color }}
              className="w-6 rounded-md border-none block"
            ></div>
            <div className="ml-2">{color}</div>
          </>
        )}
        {row("Unit", unit)}
      </ul>
    </>
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
