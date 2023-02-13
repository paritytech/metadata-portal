import { useEffect, useState } from "react";
import "./SpecsTab.css";
import { DocumentDuplicateIcon } from "@heroicons/react/24/outline";

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

export default function Hash({ value }: HashProps) {
  const [copied, setCopied] = useState<boolean>(false);

  useEffect(() => {
    setTimeout(() => copied && setCopied(false), 2000);
  }, [copied]);

  const elipsisHash = (el: string) => {
    const sliced = el.slice(0, 6) + "..." + el.slice(el.length - 4, el.length);
    const cName = copied ? "fade" : "hidden";
    return (
      <div className="flex">
        <div className="flex float-left">{sliced}</div>
        <DocumentDuplicateIcon
          className="w-5 h-5 ml-2 cursor-pointer"
          onClick={() => {
            setCopied(true);
            copyToClipboard(el);
          }}
        />
        <div className={"text-green-500 ml-3 ".concat(cName)}>Copied</div>
      </div>
    );
  };

  return elipsisHash(value);
}
