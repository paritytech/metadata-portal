import IconSrc from "../assets/icons/vault.svg";

const LINKS = [["Download", "https://www.parity.io/technologies/signer/"]];

export const AppLinks = () => {
  return (
    <div className="lg:fixed lg:top-0 lg:left-0 lg:right-0 lg:z-50 lg flex items-center space-x-4 p-4 text-sm bg-black text-white">
      <div className="flex flex-1">
        <div className="hidden lg:block w-full max-w-xs -mr-2" />
        <div className="font-bold w-full">
          <img
            src={IconSrc}
            className="relative -top-px inline-block w-6 mr-2"
          />
          <span>{"Download "}</span>
          <span className="text-pink-400">Polkadot Vault</span>
          <span className="hidden lg:inline">{" cold storage wallet app"}</span>
          <span className="hidden xl:inline">
            {" and use the portal for adding and updating networks"}
          </span>
        </div>
      </div>
      <div className="space-x-4">
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
      </div>
    </div>
  );
};
