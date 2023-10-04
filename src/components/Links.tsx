const LINKS = [["GitHub", "https://github.com/opentensor/metadata-portal"]];

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
