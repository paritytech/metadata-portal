import { QrInfo } from "../scheme";

export default function QrCode(info: QrInfo) {
  return (
    <div className="md:pb-5">
      <img src={process.env.PUBLIC_URL + info} alt="metadata qr code" />
    </div>
  );
}
