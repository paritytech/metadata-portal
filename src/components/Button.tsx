import React from 'react'


interface Props {
  onClick: () => void,
  label: string | JSX.Element,
}

export default function Button({ onClick, label}: Props): JSX.Element {
  return (
      <button
          type="button"
          onClick={onClick}
          className="px-4 py-2 text-sm font-medium text-white bg-black border-2 border-white rounded-md bg-opacity-70 hover:bg-opacity-100 focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75"
      >
        {label}
      </button>
  )
}
