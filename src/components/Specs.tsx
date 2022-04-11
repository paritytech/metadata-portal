import { ReactElement, useEffect, useState } from "react";
import { ChainSpec } from "../scheme";
import "./Specs.css";
import Copy from "../assets/copy.png";

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
  const { rpcEndpoint, genesisHash, unit, addressPrefix } = chainSpecs;

  const [copied, setCopied] = useState<boolean>(false);

  useEffect(() => {
    setTimeout(() => copied && setCopied(false), 2000);
  }, [copied]);

  const elipsisHash = (el: string) => {
    const sliced = el.slice(0, 6) + "..." + el.slice(el.length - 4, el.length);

    const cName = copied ? "fade" : "hidden";
    console.log("copied", copied);
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
    <ul>
      {row(
        "RPC endpoint",
        <div className="font-bold" style={{ color: color }}>
          {rpcEndpoint}
        </div>
      )}
      {row("Genesis hash", elipsisHash(genesisHash))}
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
      {row("Address prefix", addressPrefix.toString())}
    </ul>
  );
}

function row(title: string, content: ReactElement | string, color = "#000000") {
  return (
    <li className="py-1">
      <div className="text-sm text-black">{title}</div>
      <div className="flex" style={{ color }}>
        {content}
      </div>
    </li>
  );
}
