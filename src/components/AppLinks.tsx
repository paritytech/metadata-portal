import iconSrc from "../assets/icons/vault.svg";

const LINKS = [["Download", "https://www.parity.io/technologies/signer/"]];

export const AppLinks = () => {
  return (
    <div className="md:fixed md:top-0 md:left-0 md:right-0 md:z-50 lg flex items-center space-x-4 p-4 text-sm bg-black text-white">
      <div className="flex flex-1">
        <div className="hidden md:block w-full max-w-xs -mr-2" />
        <div className="w-full">
          <img
            src={iconSrc}
            className="relative -top-px inline-block w-6 mr-2"
          />
          <span>{"Download "}</span>
          <span className="text-pink-400">Polkadot Vault</span>
          <span className="hidden md:inline">
            {
              " cold storage wallet app and use the portal for adding and updating networks"
            }
          </span>
        </div>
      </div>
      <div className="space-x-4">
        {LINKS.map(([label, href], i) => (
          <a
            className="py-2 px-8 rounded-4xl bg-pink-600"
            href={href}
            key={i}
            target="_blank"
            rel="noreferrer"
          >
            {label}
          </a>
        ))}
      </div>
    </div>
  );
};
