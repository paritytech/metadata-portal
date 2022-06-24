import { ReactElement, useEffect, useState } from "react";
import { ChainSpec } from "../scheme";
import "./Specs.css";
import Copy from "../assets/copy.svg";

interface SpecsProps {
  chainSpecs: ChainSpec;
  color?: string;
}

export const copyToClipboard = (text: string): void => {
  const dummy = document.createElement("textarea");
  document.body.appendChild(dummy);
  dummy.value = text;
  dummy.select();
  document.execCommand("copy");
  document.body.removeChild(dummy);
};

export default function Specs({ chainSpecs, color }: SpecsProps) {
  const { rpcEndpoint, genesisHash, unit, base58prefix } = chainSpecs;
  const [copied, setCopied] = useState<boolean>(false);

  useEffect(() => {
    setTimeout(() => copied && setCopied(false), 2000);
  }, [copied]);

  const elipsisHash = (el: string) => {
    const sliced = el.slice(0, 6) + "..." + el.slice(el.length - 4, el.length);
    const cName = copied ? "fade" : "hidden";
    return (
      <>
        <div className="flex float-left">{sliced}</div>
        <img
          className="w-5 h-5 ml-2 cursor-pointer"
          src={Copy}
          onClick={() => {
            setCopied(true);
            copyToClipboard(el);
          }}
        />
        <div className={"text-green-500 ml-3 ".concat(cName)}>Copied</div>
      </>
    );
  };

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
        {row("Genesis hash", elipsisHash(genesisHash))}
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

function row(
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
