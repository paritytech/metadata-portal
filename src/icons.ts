import polkadotSrc from "./assets/icons/polkadot.svg";
import frequencySrc from "./assets/icons/frequency.svg";

const ICONS = {
  polkadot: polkadotSrc,
  frequency: frequencySrc,
  "frequency-testnet": frequencySrc,
};

export function icon(network: string) {
  return ICONS[network as "polkadot"];
}
