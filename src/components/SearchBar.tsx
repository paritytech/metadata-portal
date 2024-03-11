import { cn } from "../utils";
import { ChangeEventHandler } from "react";

export const SearchBar = ({
  searchString,
  setSearchString,
  onChange,
}: {
  searchString: string;
  setSearchString: (v: string) => void;
  onChange: ChangeEventHandler<HTMLInputElement>;
}) => {
  return (
    <div>
      <div className="relative w-full">
        <div className="absolute inset-y-0 left-0 flex items-center pl-3">
          <svg
            aria-hidden="true"
            className="w-5 h-5 text-gray-500 dark:text-gray-400"
            fill="currentColor"
            viewBox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fillRule="evenodd"
              d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
              clipRule="evenodd"
            ></path>
          </svg>
        </div>
        <input
          type="text"
          className="mb-4 p-2 w-full text-gray-900 rounded-lg text-lg focus-visible:outline-none block pl-12 pr-8"
          placeholder="Search"
          value={searchString}
          onChange={onChange}
        />
        <button
          type="button"
          className={cn(
            "absolute inset-y-0 right-0 flex items-center mr-8",
            !searchString && "hidden"
          )}
          onClick={() => setSearchString("")}
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
              fill="black"
            />
          </svg>
        </button>
      </div>
    </div>
  );
};
