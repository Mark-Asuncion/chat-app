import React from 'react';

interface InboxEntriesProps {
    // children?:      JSX.Element | JSX.Element[],
    className?:    string,
}

export default function InboxEntries(props: InboxEntriesProps) {
    const classN = ( props.className )? ` ${props.className}`:"";
    return (
        <div
            className={ 'shadow-lg rounded-md p-2'
                + ' ' + classN }>
        </div>
    )
}
