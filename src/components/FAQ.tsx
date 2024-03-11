import { Disclosure } from "@headlessui/react";
import { ReactNode } from "react";
import { ChevronIcon } from "./ChevronIcon";

const OrdinalNumber = ({ children }: { children: ReactNode }) => (
  <div className="w-8 h-8 rounded-full border border-neutral-200 text-black text-opacity-40 flex flex-shrink-0 items-center justify-center mr-4">
    {children}
  </div>
);

export const FAQ = () => (
  <div>
    <h3 className="text-[40px] py-8 unbounded">FAQ</h3>
    <div className="space-y-2">
      <Disclosure as="div" className="border border-neutral-200 rounded-4xl">
        <Disclosure.Button className="flex items-center justify-between w-full p-4 pl-6 space-x-4">
          <h4 className="md:text-2xl text-left">How Can I Use The Portal?</h4>
          <ChevronIcon />
        </Disclosure.Button>
        <Disclosure.Panel className="px-6 pt-0 pb-8 max-w-3xl">
          <p>
            The Portal is primarily used to add and update networks for{" "}
            <strong className="font-bold">
              Polkadot Vault cold storage wallet
            </strong>
            . Since Polkadot Vault is always offline and air-gapped, you need to
            follow a <strong className="font-bold">unique process</strong> to
            update the chain metadata so that your transactions are valid.
          </p>
        </Disclosure.Panel>
      </Disclosure>
      <Disclosure as="div" className="border border-neutral-200 rounded-4xl">
        <Disclosure.Button className="flex items-center justify-between w-full p-4 pl-6 space-x-4">
          <h4 className="md:text-2xl text-left">What Is Polkadot Vault?</h4>
          <ChevronIcon />
        </Disclosure.Button>
        <Disclosure.Panel className="px-6 pt-0 pb-8 max-w-3xl">
          <p>
            Polkadot Vault is a cold storage solution that turns your iOS or
            Android device into a dedicated hardware wallet for Polkadot,
            Kusama, and other Substrate-based chains. Your keys are kept secure
            (i.e. offline) at all times, and transactions are signed in an
            air-gapped way via QR-codes.
          </p>
        </Disclosure.Panel>
      </Disclosure>
      <Disclosure as="div" className="border border-neutral-200 rounded-4xl">
        <Disclosure.Button className="flex items-center justify-between w-full p-4 pl-6 space-x-4">
          <h4 className="md:text-2xl text-left">
            How To Add And Update Networks In Polkadot Vault
          </h4>
          <ChevronIcon />
        </Disclosure.Button>
        <Disclosure.Panel className="px-6 pt-0 pb-8 max-w-3xl">
          <ol className="space-y-2">
            <li className="flex items-top">
              <OrdinalNumber>1</OrdinalNumber>
              <div className="pt-1">
                The first step is to locate the network that needs metadata
                updating in <strong className="font-bold">Parity</strong> or{" "}
                <strong className="font-bold">Novasama</strong>. Note: To
                navigate between the two portals, use the Metadata Portal
                dropdown at the top of the page.
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>2</OrdinalNumber>
              <div className="pt-1">
                Then select the{" "}
                <strong className="font-bold">{'"Chain Spec"'}</strong> tab
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>3</OrdinalNumber>
              <div className="pt-1">
                Open the scanner tab from your Polkadot Vault device and scan
                the network’s{" "}
                <strong className="font-bold">{'"Chain Spec" QR code'}</strong>
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>4</OrdinalNumber>
              <div className="pt-1">
                Review the verifier certificate, and{" "}
                <strong className="font-bold">{'Select "Approve"'}</strong>
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>5</OrdinalNumber>
              <div className="pt-1">
                Select the{" "}
                <strong className="font-bold">{'"Metadata" tab'}</strong> at the
                top of the metadata portal screen
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>6</OrdinalNumber>
              <div className="pt-1">
                Open the scanner tab in your Polkadot Vault device again and{" "}
                <strong className="font-bold">
                  {'scan the network’s "Chain Spec" QR code'}
                </strong>
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>7</OrdinalNumber>
              <div className="pt-1">
                <strong className="font-bold">Scan the Metadata QR code</strong>
                . Note: this can take a few minutes to complete.
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>8</OrdinalNumber>
              <div className="pt-1">
                Finally, review the verifier certificate and{" "}
                <strong className="font-bold">{'Select "Approve"'}</strong>
              </div>
            </li>
          </ol>
        </Disclosure.Panel>
      </Disclosure>
      <Disclosure as="div" className="border border-neutral-200 rounded-4xl">
        <Disclosure.Button className="flex items-center justify-between w-full p-4 pl-6 space-x-4">
          <h4 className="md:text-2xl text-left">
            {"I Haven't Found The Network I Need"}
          </h4>
          <ChevronIcon />
        </Disclosure.Button>
        <Disclosure.Panel className="px-6 pt-0 pb-8 max-w-3xl">
          <p className="pb-4">
            Metadata about networks chain specs is stored in two places:
          </p>
          <ol className="space-y-2 pb-4">
            <li className="flex items-top">
              <OrdinalNumber>1</OrdinalNumber>
              <div className="pt-1">
                <a
                  href="https://metadata.parity.io/#/polkadot"
                  rel="noopener noreferrer"
                  className="font-bold underline"
                >
                  Parity
                </a>{" "}
                for Polkadot, Kusama, and Westend
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>2</OrdinalNumber>
              <div className="pt-1">
                <a
                  href="https://centrifuge.github.io/metadata-portal/"
                  rel="noopener noreferrer"
                  className="font-bold underline"
                >
                  Centrifuge
                </a>{" "}
                for Centrifuge and Altair parachains
              </div>
            </li>
            <li className="flex items-top">
              <OrdinalNumber>2</OrdinalNumber>
              <div className="pt-1">
                <a
                  href="https://metadata.novasama.io/#/darwinia%20parachain"
                  rel="noopener noreferrer"
                  className="font-bold underline"
                >
                  Novasama
                </a>{" "}
                for other Parachains and Solochains
              </div>
            </li>
          </ol>
          <p>
            If you cannot find the network you are looking for, contact the
            network’s developers directly
          </p>
        </Disclosure.Panel>
      </Disclosure>
    </div>
  </div>
);
