const LINKS = [
  ["GitHub", "https://github.com/paritytech/metadata-portal"],
  ["Terms of Service", "https://www.parity.io/terms/"],
];

export const Links = () => {
  return (
    <div className="flex space-x-2 text-neutral-400">
      {LINKS.map(([label, href], i) => (
        <a className="bordered-action" href={href} key={i}>
          {label}
        </a>
      ))}
    </div>
  );
};
