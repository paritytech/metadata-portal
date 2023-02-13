import { ReactNode } from "react";
import { ChainSpec } from "../scheme";
import Hash from "./Hash";
import "./SpecsTab.css";

export default function SpecsTab({ specs }: { specs: ChainSpec }) {
  const { color, rpcEndpoint, genesisHash, unit, base58prefix, specsQr } =
    specs;

  return (
    <div className="space-y-4">
      <div className="flex flex-col items-center text-sm space-y-2">
        <img
          className="w-full"
          src={process.env.PUBLIC_URL + specsQr.path}
          alt="Qr code"
        />
        <div>
          {"Scan this code to add chain specs to the "}
          <a
            href="https://parity.io/signer/"
            className="font-bold"
            style={{ color }}
            target="_blank"
            rel="noreferrer"
          >
            Parity Signer App
          </a>
        </div>
      </div>
      <div>
        <ul>
          <Row title="RPC endpoint">{rpcEndpoint}</Row>
          <Row title="Genesis hash">
            <Hash value={genesisHash} />
          </Row>
          <Row title="Address prefix">{base58prefix}</Row>
          <Row title="Color">
            <div className="flex space-x-2">
              <div className="ml-2">{color}</div>
              <div
                style={{ backgroundColor: color }}
                className="w-6 rounded-md"
              />
            </div>
          </Row>
          <Row title="Unit">{unit}</Row>
        </ul>
      </div>
    </div>
  );
}

export function Row({
  title,
  children,
  color = "#000000",
}: {
  title: string;
  children?: ReactNode;
  color?: string;
}) {
  return (
    <li className="flex py-1 justify-between text-sm">
      <div>{title}</div>
      <div style={{ color }}>{children}</div>
    </li>
  );
}
