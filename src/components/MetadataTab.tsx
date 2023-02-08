import {
  ChainSpec,
  QrInfo,
  RpcSource,
  SourceType,
  WasmSource,
} from "../scheme";
import Hash from "./Hash";
import { row } from "./SpecsTab";
import { Fragment, useState } from "react";
import { Listbox, Transition } from "@headlessui/react";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/24/solid";
import { ArrowTopRightOnSquareIcon } from "@heroicons/react/24/outline";

type LabeledQr = {
  qr: QrInfo;
  label: string;
};

interface MetadataTabProps {
  specs: ChainSpec;
}

export default function MetadataTab({ specs }: MetadataTabProps) {
  const {
    color,
    metadataQr,
    metadataVersion,
    nextMetadataVersion,
    nextMetadataQr,
  } = specs;

  const qrs: LabeledQr[] = [
    { qr: metadataQr, label: `Current: #${metadataVersion}` },
  ];
  if (nextMetadataVersion && nextMetadataQr) {
    qrs.push({ qr: nextMetadataQr, label: `Next #${nextMetadataVersion}` });
  }
  const [selectedIdx, setSelectedIdx] = useState(0);

  return (
    <div className="w-fit">
      <Listbox value={selectedIdx} onChange={setSelectedIdx}>
        <div className="relative mt-4 w-full">
          <Listbox.Button className="border border-gray-200 relative w-full cursor-default rounded-lg bg-white py-2 pl-3 pr-10 text-left shadow-md focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300">
            <span className="block truncate">
              <span className="font-bold">{qrs[selectedIdx].label}</span>
              <SignedBy signer={qrs[selectedIdx].qr.signedBy} />
            </span>
            <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
              <ChevronUpDownIcon
                className="h-5 w-5 text-gray-400"
                aria-hidden="true"
              />
            </span>
          </Listbox.Button>
          <Transition
            as={Fragment}
            leave="transition ease-in duration-100"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <Listbox.Options className="absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
              {qrs.map((qr, idx) => (
                <Listbox.Option
                  key={idx}
                  className={({ active }) =>
                    `relative cursor-default select-none py-2 pl-10 pr-4 ${
                      active ? "bg-amber-100 text-amber-900" : "text-gray-900"
                    }`
                  }
                  value={idx}
                >
                  {({ selected }) => (
                    <>
                      <span
                        className={`block truncate ${
                          selected ? "font-medium" : "font-normal"
                        }`}
                      >
                        {qr.label}
                        <SignedBy signer={qr.qr.signedBy} />
                      </span>
                      {selected ? (
                        <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-amber-600">
                          <CheckIcon className="h-5 w-5" aria-hidden="true" />
                        </span>
                      ) : null}
                    </>
                  )}
                </Listbox.Option>
              ))}
            </Listbox.Options>
          </Transition>
        </div>
      </Listbox>

      <div className="pt-10 md:px-4 text-center md:text-left">
        {"Scan this QR video to update "}
        <a
          href="https://parity.io/signer/"
          target="_blank"
          className="font-medium"
          style={{ color: `${color}` }}
          rel="noreferrer"
        >
          Parity Signer App
          <ArrowTopRightOnSquareIcon className={"inline w-4 h-4 ml-2"} />
        </a>
      </div>

      <div className="md:w-[468px] md:h-[468px]">
        <img
          src={process.env.PUBLIC_URL + qrs[selectedIdx].qr.path}
          alt="metadata qr code"
        />
      </div>
      <div className="px-4">
        <SourceBlock source={qrs[selectedIdx].qr.source} />
      </div>
    </div>
  );
}

interface SourceBlockProps {
  source: SourceType;
}

function SourceBlock({ source }: SourceBlockProps) {
  if (source) {
    switch (source.type) {
      case "Wasm": {
        const s = source as WasmSource;
        return (
          <ul className="flex">
            {row(
              "Metadata source",
              <a
                href={`https://github.com/${s.github_repo}/releases`}
                target="_blank"
                rel="noreferrer"
              >
                {s.github_repo}
                <ArrowTopRightOnSquareIcon className={"inline w-4 h-4 ml-1"} />
              </a>
            )}
            {row("Blake2-256 hash", <Hash value={s.hash} />)}
          </ul>
        );
      }
      case "Rpc": {
        const s = source as RpcSource;
        return (
          <ul className="flex">
            {row("Source block", <Hash value={s.block} />, true)}
          </ul>
        );
      }
    }
  }
  return null;
}

interface SignedByProps {
  signer: string | null;
}

function SignedBy({ signer }: SignedByProps) {
  return signer ? (
    <span className="p-1 mx-2 rounded-full text-xs text-green-700 bg-green-100">
      Signed by {signer}
    </span>
  ) : (
    <span className="p-1 mx-2 rounded-full text-red-700 bg-red-100">
      Unsigned
    </span>
  );
}
