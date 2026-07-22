import polkadotSrc from "./assets/icons/polkadot.svg";
import assetHubSrc from "./assets/icons/asset-hub.svg";
import bridgeHubSrc from "./assets/icons/bridge-hub.svg";
import bridgeHubBlackSrc from "./assets/icons/bridge-hub-black.svg";
import collectivesSrc from "./assets/icons/collectives.svg";
import kusamaSrc from "./assets/icons/kusama.svg";
import westendSrc from "./assets/icons/westend.svg";
import { nodesEncointerBlueSVG } from "./assets/icons/encointer-SVG";
import peopleSrc from "./assets/icons/people.svg";
import coretimeSrc from "./assets/icons/coretime.svg";
import bulletinSrc from "./assets/icons/bulletin.svg";
import paseoSrc from "./assets/icons/paseo.svg";

const ICONS = {
  polkadot: polkadotSrc,
  "polkadot-statemint": assetHubSrc,
  "polkadot-bridge-hub-polkadot": bridgeHubSrc,
  "polkadot-collectives": collectivesSrc,
  "polkadot-coretime-polkadot": coretimeSrc,
  "polkadot-people-polkadot": peopleSrc,
  "polkadot-bulletin-polkadot": bulletinSrc,
  kusama: kusamaSrc,
  "kusama-statemine": assetHubSrc,
  "kusama-bridge-hub-kusama": bridgeHubBlackSrc,
  "kusama-coretime-kusama": coretimeSrc,
  "kusama-encointer-parachain": nodesEncointerBlueSVG,
  "kusama-people-kusama": peopleSrc,
  westend: westendSrc,
  "westend-westmint": assetHubSrc,
  "westend-bridge-hub-westend": bridgeHubSrc,
  "westend-collectives-westend": collectivesSrc,
  "westend-coretime-westend": coretimeSrc,
  "westend-people-westend": peopleSrc,
  paseo: paseoSrc,
  "paseo-asset-hub-paseo": assetHubSrc,
  "paseo-people-paseo": peopleSrc,
};

export function icon(network: string) {
  return ICONS[network as "polkadot"];
}
