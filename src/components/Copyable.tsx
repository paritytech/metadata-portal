import { useEffect, useState } from "react";
import "./SpecsTab.css";
import { DocumentDuplicateIcon } from "@heroicons/react/24/outline";

interface CopyableProps {
  value: string;
  slicer: (v: string) => string;
}

export const copyToClipboard = (text: string): void => {
  const dummy = document.createElement("textarea");
  document.body.appendChild(dummy);
  dummy.value = text;
  dummy.select();
  document.execCommand("copy");
  document.body.removeChild(dummy);
};

export const hashSlicer = (v: string) =>
  v.slice(0, 6) + "..." + v.slice(v.length - 4, v.length);
export const keepHeadSlicer = (i: number) => (v: string) => {
  if (v.length <= i) {
    return v;
  }
  return v.slice(0, i) + "...";
};

export default function Copyable({ value, slicer }: CopyableProps) {
  const [copied, setCopied] = useState<boolean>(false);

  useEffect(() => {
    setTimeout(() => copied && setCopied(false), 2000);
  }, [copied]);

  const copyable = (el: string) => {
    const sliced = slicer(el);
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

  return copyable(value);
}
