import { Portals } from "./scheme";

export function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

export function cn(...classes: (string | boolean | undefined)[]) {
  return classes.filter(Boolean).join(" ");
}

export function currentPortalKey(portals: Portals) {
  const keys = Object.keys(portals);

  return (
    keys.find((key) => new URL(portals[key].url).host === location.host) ||
    keys[2]
  );
}
