import { ReactNode } from "react";

export function Row({
  title,
  children,
}: {
  title: string;
  children?: ReactNode;
}) {
  return (
    <li className="flex justify-between space-x-8">
      <div className="text-neutral-500">{title}</div>
      <div className="text-right">{children}</div>
    </li>
  );
}
