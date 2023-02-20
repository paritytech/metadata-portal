const LINKS = [["Download", "https://www.parity.io/technologies/signer/"]];

export const AppLinks = () => {
  return (
    <div className="flex justify-between p-4 text-sm bg-black text-white">
      <span className="font-bold">
        Get <span className="text-pink-400">Polkadot Vault</span>
      </span>
      <span className="space-x-4">
        {LINKS.map(([label, href], i) => (
          <a
            className="py-2 px-8 rounded-4xl bg-pink-600 font-bold"
            href={href}
            key={i}
            target="_blank"
            rel="noreferrer"
          >
            {label}
          </a>
        ))}
      </span>
    </div>
  );
};
