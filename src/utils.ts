export function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}

export function getBackgroundStyle(color: string) {
  return color.includes("linear-gradient")
    ? { backgroundImage: color }
    : { backgroundColor: color };
}
