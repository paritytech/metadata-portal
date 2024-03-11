import polkadotSrc from "./assets/icons/polkadot.svg";
import assetHubSrc from "./assets/icons/asset-hub.svg";
import bridgeHubSrc from "./assets/icons/bridge-hub.svg";
import bridgeHubBlackSrc from "./assets/icons/bridge-hub-black.svg";
import collectivesSrc from "./assets/icons/collectives.svg";
import kusamaSrc from "./assets/icons/kusama.svg";
import westendSrc from "./assets/icons/westend.svg";
import rococoSrc from "./assets/icons/rococo.svg";
import { nodesEncointerBlueSVG } from "./assets/icons/encointer-SVG";
import { contractsPNG } from "./assets/icons/contracts-PNG";

const ICONS = {
  polkadot: polkadotSrc,
  "polkadot-statemint": assetHubSrc,
  "polkadot-bridge-hub-polkadot": bridgeHubSrc,
  "polkadot-collectives": collectivesSrc,
  kusama: kusamaSrc,
  "kusama-statemine": assetHubSrc,
  "kusama-bridge-hub-kusama": bridgeHubBlackSrc,
  "kusama-encointer-parachain": nodesEncointerBlueSVG,
  westend: westendSrc,
  "westend-westmint": assetHubSrc,
  "westend-collectives": collectivesSrc,
  rococo: rococoSrc,
  "rococo-statemine": assetHubSrc,
  "rococo-bridge-hub-rococo": bridgeHubBlackSrc,
  "rococo-contracts-rococo": contractsPNG,
  "rococo-encointer-parachain": nodesEncointerBlueSVG,
};

export function icon(network: string) {
  return ICONS[network as "polkadot"];
}
