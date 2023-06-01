import { Portals } from "./scheme";

export function title(input: string) {
  // Split the input string into an array of words
  const words = input.split("-");

  // Capitalize each word and join them with spaces
  const convertedString = words
    .map((word: string) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");

  return convertedString;
}

export function cn(...classes: (string | boolean | undefined)[]) {
  return classes.filter(Boolean).join(" ");
}

export function currentPortalKey(portals: Portals) {
  const keys = Object.keys(portals);

  return (
    keys.find((key) => new URL(portals[key].url).host === location.host) ||
    keys[0]
  );
}
