import iconSrc from "../assets/icons/vault.svg";
import { useEffect, useState } from "react";

const LINK = "https://www.parity.io/technologies/signer/";

const HIDE_BANNER_KEY = "hideBanner";

export const Banner = () => {
  const [hideBanner, setHideBanner] = useState(false);

  useEffect(() => {
    // Check if there is any saved state in local storage
    const savedState = localStorage.getItem(HIDE_BANNER_KEY) === "true";
    if (savedState) {
      setHideBanner(savedState);
    }
  }, []);

  useEffect(() => {
    // Save the state to local storage whenever it changes
    localStorage.setItem(HIDE_BANNER_KEY, hideBanner.toString());
  }, [hideBanner]);

  if (hideBanner) return null;

  return (
    <div className="flex items-center space-x-4 p-4 text-sm bg-black text-white">
      <div className="flex flex-1">
        <div className="hidden xl:block w-full max-w-xs -mr-2" />
        <div className="w-full">
          <img
            src={iconSrc}
            className="relative -top-px inline-block w-6 mr-2"
            alt="Polkadot Vault logo"
          />
          <span>{"Download "}</span>
          <a
            className="text-pink-400"
            href={LINK}
            target="_blank"
            rel="noreferrer"
          >
            Polkadot Vault
          </a>
          <span className="hidden md:inline">
            {
              " cold storage wallet app and use the portal for adding and updating networks"
            }
          </span>
        </div>
      </div>
      <div className="space-x-4">
        <a
          className="py-2 px-8 rounded-4xl bg-pink-600"
          href={LINK}
          target="_blank"
          rel="noreferrer"
        >
          Download
        </a>
      </div>
      <button
        type="button"
        className="items-center mr-8"
        onClick={() => setHideBanner(true)}
      >
        <svg
          width="14"
          height="14"
          viewBox="0 0 14 14"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M14 1.41L12.59 0L7 5.59L1.41 0L0 1.41L5.59 7L0 12.59L1.41 14L7 8.41L12.59 14L14 12.59L8.41 7L14 1.41Z"
            fill="white"
          />
        </svg>
      </button>
    </div>
  );
};
