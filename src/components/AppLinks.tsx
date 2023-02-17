const STATESMENT = "Parity Signer";
const LINKS = [
  ["iOS", "https://itunes.apple.com/us/app/parity-signer/id1218174838"],
  ["Android", "https://play.google.com/store/apps/details?id=io.parity.signer"],
];

export const AppLinks = () => {
  return (
    <div className="flex justify-end p-2 space-x-4 text-sm bg-black text-white">
      <span>{STATESMENT}</span>
      {LINKS.map(([label, href], i) => (
        <a href={href} key={i}>
          {label}
        </a>
      ))}
    </div>
  );
};
