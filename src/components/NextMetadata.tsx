import { Dialog, Transition } from "@headlessui/react";
import { Fragment, useState } from "react";
import React from "react";
import {ChainSpec} from "../scheme";

export default function NextMetadata(chainSpec: ChainSpec) {
  const [isOpen, setIsOpen] = useState(false);

  function closeModal() {
    setIsOpen(false);
  }

  function openModal() {
    setIsOpen(true);
  }

  if (!chainSpec.nextMetadataVersion) {
    return null;
  }

  const qrPath = chainSpec.nextMetadataQr?.path

  return (
    <>
      <div className="flex items-center mt-5">
        <button
            type="button"
            onClick={openModal}
            className="px-4 py-2 text-sm font-medium text-white bg-green-600 border-2 border-white rounded-md hover:bg-opacity-70 focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75"
        >
          {`Metadata ${chainSpec.nextMetadataVersion} is available!`}
        </button>
      </div>

      <Transition appear show={isOpen} as={Fragment}>
        <Dialog
          as="div"
          className="fixed inset-0 z-10 overflow-y-auto bg-black/90 backdrop-blur"
          onClose={closeModal}
        >
          <div className="px-4 text-center">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0"
              enterTo="opacity-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100"
              leaveTo="opacity-0"
            >
              <Dialog.Overlay className="fixed inset-0" />
            </Transition.Child>

            {/* This element is to trick the browser into centering the modal contents. */}
            <span
              className="inline-block h-screen align-middle"
              aria-hidden="true"
            >
              &#8203;
            </span>
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 scale-95"
              enterTo="opacity-100 scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 scale-100"
              leaveTo="opacity-0 scale-95"
            >
              <div className="opacity-100 inline-block h-4/5 p-6 my-8 overflow-hidden text-center align-middle transition-all transform bg-white shadow-xl rounded-2xl">
                <Dialog.Title
                  as="h3"
                  className="text-lg font-medium leading-6 text-gray-900"
                >
                  ⚠️ This metadata is not on chain yet! ⚠️
                </Dialog.Title>
                <div className="min-h-[180px]">
                  <img src={process.env.PUBLIC_URL + qrPath} className="ml-auto mr-auto" alt="Qr code" />
                </div>

                <div>
                  <button
                    type="button"
                    className="inline-flex justify-center px-4 py-2 text-sm font-medium text-blue-900 bg-blue-100 border border-transparent rounded-md hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-500"
                    onClick={closeModal}
                  >
                    Got it, thanks!
                  </button>
                </div>
              </div>
            </Transition.Child>
          </div>
        </Dialog>
      </Transition>
    </>
  );
}
