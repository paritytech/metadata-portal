import { useEffect, useState } from "react";
import "./Specs.css";
import Copy from "../assets/copy.svg";

interface HashProps {
  value: string;
}

export const copyToClipboard = (text: string): void => {
  const dummy = document.createElement("textarea");
  document.body.appendChild(dummy);
  dummy.value = text;
  dummy.select();
  document.execCommand("copy");
  document.body.removeChild(dummy);
};

export default function Hash({value}: HashProps) {
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
         alt="copy" />
        <div className={"text-green-500 ml-3 ".concat(cName)}>Copied</div>
      </>
    );
  };

  return elipsisHash(value);
}

