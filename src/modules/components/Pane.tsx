import React from 'react';

interface PaneProps {
    children:      JSX.Element | JSX.Element[],
    className?:    string,
}

export default function Pane(props: PaneProps) {
    const classN = ( props.className )? ` ${props.className}`:"";
    return (
        <div
            className={ 'bg-neutral-900 shadow-lg shadow-black'
                + ' rounded-md h-full p-3' + classN }>
            {props.children}
        </div>
    )
}
