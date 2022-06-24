import {QrInfo, RpcSource, WasmSource} from "../scheme";

export default function QrCode({ path, source }: QrInfo) {
  let source_block = null;
  if (source) {
    switch (source.type) {
      case "Wasm": {
        const s = source as WasmSource;
        source_block = (
            <div>
              Source:
              Repo: {s.github_repo}
              Hash: {s.hash}
            </div>
        )
        break
      }
      case "Rpc": {
        const s = source as RpcSource;
        source_block = (
            <div>
              <p>{"Source:"}</p>
              <p>{"Url: "} {s.url}</p>
              <p>{"Block: "} {s.block}</p>
            </div>
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
