export const Headline = ({ heading }: { heading: string }) => {
  return (
    <div className="flex items-center justify-between py-4">
      <h1 className="font-bold text-4xl">{heading}</h1>
      <div className="space-x-4">
        <a
          href="https://github.com/paritytech/metadata-portal"
          target="blank"
          rel="noreferrer"
          className="border border-neutral-200 hover:border-neutral-400 px-2 py-1 rounded-full transition-colors"
        >
          GitHub
        </a>
        <a
          href="https://www.parity.io/terms/"
          target="_blank"
          rel="noreferrer"
          className="border border-neutral-200 hover:border-neutral-400 px-2 py-1 rounded-full transition-colors"
        >
          Terms & Services
        </a>
      </div>
    </div>
  );
};
