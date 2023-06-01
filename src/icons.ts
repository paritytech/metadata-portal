import polkadotSrc from "./assets/icons/polkadot.svg";
import kusamaSrc from "./assets/icons/kusama.svg";
import westendSrc from "./assets/icons/westend.svg";
import rococoSrc from "./assets/icons/rococo.svg";
import frequencySrc from "./assets/icons/frequency.svg";

const ICONS = {
  polkadot: polkadotSrc,
  kusama: kusamaSrc,
  westend: westendSrc,
  rococo: rococoSrc,
  frequency: frequencySrc,
  frequency_rococo: frequencySrc,
};

export function icon(network_name: string) {
  const network = network_name.replace(/-/g, "_");
  return ICONS[network as "polkadot"];
}
