import { ChainSpec } from "../scheme";

export default function Specs(chainSpec: ChainSpec) {
  return (
    <ul>
      {row("RPC endpoint", chainSpec.rpcEndpoint)}
      {row("Genesis hash", chainSpec.genesisHash)}
      {row("Color", chainSpec.color)}
      {row("Unit", chainSpec.unit)}
      {row("Address prefix", chainSpec.addressPrefix)}
    </ul>
  );
}

function row(title: string, value: string | number) {
  return (
    <li className="py-1">
      <div className="text-sm text-neutral-300">{title}</div>
      {value}
    </li>
  );
}
