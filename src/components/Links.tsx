const LINKS = [
  ["GitHub", "https://github.com/frequency-chain/metadata-portal"],
  ["Terms of Service", "https://www.parity.io/terms/"],
];

export const Links = () => {
  return (
    <div className="flex space-x-2 text-black opacity-70">
      {LINKS.map(([label, href], i) => (
        <a
          className="bordered-action hover:bg-neutral-100 transition-colors"
          href={href}
          target="_blank"
          rel="noreferrer"
          key={i}
        >
          {label}
        </a>
      ))}
    </div>
  );
};
