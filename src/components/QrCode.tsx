import {QrInfo, RpcSource, WasmSource} from "../scheme";
import Hash from "./Hash";
import {row} from "./Specs";

export default function QrCode({ path, source }: QrInfo) {
  let source_block = null;
  if (source) {
    switch (source.type) {
      case "Wasm": {
        const s = source as WasmSource;
        source_block = (
            <ul className="flex">
              {row("Metadata source", <a href={`https://github.com/${s.github_repo}/releases`} target="_blank" rel="noreferrer">{s.github_repo}</a>)}
              {row("Blake2-256 hash", <Hash value={s.hash} />)}
            </ul>
        )
        break
      }
      case "Rpc": {
        const s = source as RpcSource;
        source_block = (
            <ul className="flex">
              {row("Source block", <Hash value={s.block} />, true)}
            </ul>
        )
        break
      }
    }
  }
  return (
    <div className="md:pb-5">
      <img src={process.env.PUBLIC_URL + path} alt="metadata qr code" />
      {source_block}
    </div>
  );
}
