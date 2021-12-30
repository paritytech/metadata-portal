import { Menu, Transition } from '@headlessui/react'
import { Fragment} from 'react'
import {ChevronDownIcon} from '@heroicons/react/solid'
import {Link} from "react-router-dom";
import {Chains} from "./data";

interface Props{
    allChains: Chains
    selectedName: string
}

export default function Selector({selectedName, allChains}: Props) {
    const selected = allChains[selectedName]
    const dropdownItems = Object.keys(allChains).filter(name => name !== selectedName).map(name => {
        return <Menu.Item key={name}>
            {({ active }) => (
                <Link to={`/${name}`}>
                    <button
                        className={`${
                            active ? 'bg-gray-200' : 'text-gray-900'
                        } group flex rounded-md items-center w-full px-2 py-2 text-base`}
                    >
                        <img className="w-5 h-5 mr-2" src={allChains[name].iconPath} alt="icon" />
                        {name}
                    </button>
                </Link>
            )}
        </Menu.Item>
    })
    return (
        <div className="text-right">
            <Menu as="div" className="relative inline-block text-left w-48 text-2xl">
                <div>
                    <Menu.Button className="inline-flex justify-center w-full items-center px-4 py-2 bg-white rounded-md border-2 border-black hover:bg-gray-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75">
                        <img className="w-5 h-5 mr-2" src={selected.iconPath} alt="icon" />
                        {selected.name}
                        <ChevronDownIcon
                            className="w-5 h-5 ml-2"
                            aria-hidden="true"
                        />
                    </Menu.Button>
                </div>
                <Transition
                    as={Fragment}
                    enter="transition ease-out duration-100"
                    enterFrom="transform opacity-0 scale-95"
                    enterTo="transform opacity-100 scale-100"
                    leave="transition ease-in duration-75"
                    leaveFrom="transform opacity-100 scale-100"
                    leaveTo="transform opacity-0 scale-95"
                >

                    <Menu.Items className="absolute right-0 w-full mt-2 origin-top-right bg-white divide-y divide-gray-100 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                        <div className="px-1 py-1 ">
                            {dropdownItems}
                        </div>
                    </Menu.Items>
                </Transition>
            </Menu>
        </div>
    )
}
