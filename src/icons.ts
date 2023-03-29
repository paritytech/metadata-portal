import polkadotSrc from "./assets/icons/polkadot.svg";
import kusamaSrc from "./assets/icons/kusama.svg";
import westendSrc from "./assets/icons/westend.svg";

const ICONS = {
  polkadot: polkadotSrc,
  kusama: kusamaSrc,
  westend: westendSrc,
};

export function icon(network: string) {
  return ICONS[network as "polkadot"];
}
