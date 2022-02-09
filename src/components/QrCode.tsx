import { QrInfo } from "../scheme";
import { BadgeCheckIcon, ExclamationCircleIcon } from "@heroicons/react/solid";

export default function QrCode({ path, signedBy }: QrInfo) {
  const svgClass = "inline mr-2 h-7";
  return (
    <div className="shadow-xl pb-5 bg-white">
      <img src={process.env.PUBLIC_URL + path} alt="metadata qr code" />
      <div className="flex justify-center text-lg font-medium">
        {signedBy ? (
          <div className="text-green-700">
            <BadgeCheckIcon className={svgClass} />
            Signed by {signedBy}
          </div>
        ) : (
          <div className="text-red-500">
            <ExclamationCircleIcon className={svgClass} />
            Unsigned
          </div>
        )}
      </div>
    </div>
  );
}
