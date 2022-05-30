import { QrInfo } from "../scheme";

export default function QrCode({ path }: QrInfo) {
  return (
    <div className="md:pb-5">
      <img src={process.env.PUBLIC_URL + path} alt="metadata qr code" />
    </div>
  );
}
