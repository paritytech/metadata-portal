import {QrInfo} from "../data";
import {BadgeCheckIcon, ShieldExclamationIcon} from "@heroicons/react/solid";


export default function QrCode({path, isVerified}: QrInfo) {
    const svgClass = "inline mr-2 h-7"
    return (
        <div className="pb-5 bg-white">
            <img src={process.env.PUBLIC_URL + path} alt="metadata qr code"/>
            <div className="flex justify-center text-lg font-medium">
                {isVerified ? (
                        <div className="text-green-700">
                            <BadgeCheckIcon className={svgClass}/>
                            Signed by Parity
                        </div>
                    )
                : (
                    <div className="text-amber-500">
                        <ShieldExclamationIcon className={svgClass}/>
                            Unsigned
                    </div>
                    )

                }
            </div>
        </div>
    )
}
