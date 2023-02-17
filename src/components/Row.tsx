import { ReactNode } from "react";

export function Row({
  title,
  children,
}: {
  title: string;
  children?: ReactNode;
}) {
  return (
    <li className="flex justify-between">
      <div className="text-neutral-500">{title}</div>
      <div>{children}</div>
    </li>
  );
}
