import polkadotSrc from "./assets/icons/polkadot.svg";
import frequencySrc from "./assets/icons/frequency.svg";
import frequencyTestnetSrc from "./assets/icons/frequency-testnet.svg";

const ICONS = {
  polkadot: polkadotSrc,
  frequency: frequencySrc,
  "frequency-testnet": frequencyTestnetSrc,
};

export function icon(network: string) {
  return ICONS[network as "polkadot"];
}
